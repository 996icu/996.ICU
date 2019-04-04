package main

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"github.com/google/go-github/github"
	"golang.org/x/oauth2"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"path"
	"regexp"
	"strings"
	"sync"
)

var fakeRepository = make(chan string, 20)

const githubPrefix = "(https://github.com/"
const githubPrefixLen = len(githubPrefix)
const projectPath = "/awesomelist/projects.md"

var TOKEN = ""

func init() {
	TOKEN = os.Getenv("GH_TOKEN")
	if TOKEN == "" {
		cwd, _ := os.Getwd()
		data, err := ioutil.ReadFile(cwd + "/awesomelist/fake_license_check/.token")
		if err != nil {
			log.Fatal("failed get github token")
		}
		TOKEN = string(data)
	}
}
func main() {
	cwd, _ := os.Getwd()
	filePath := projectPath
	filePath = path.Join(cwd, filePath)
	log.Println("cwd", cwd, filePath)
	data, err := ioutil.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}
	content := string(data)
	reg := regexp.MustCompile(`\(https://github.com/.*?/.*?\)`)
	strArr := reg.FindAllString(content, -1)
	strArr = RemoveDuplicatesAndEmpty(strArr)
	log.Printf("%q\n", strArr)

	chanlen := 10
	check := make(chan string, chanlen)
	var wg sync.WaitGroup
	processChan := make(chan struct{})
	go processFakeRepo(processChan, content)
	for i := 0; i < chanlen; i++ {
		wg.Add(1)
		go processUrl(check, &wg)
	}
	for idx, str := range strArr {
		fmt.Println(idx, str)
		str = str[githubPrefixLen : len(str)-1]
		check <- str
		if idx > 5 {
			break
		}
	}
	close(check)
	wg.Wait()
	log.Println("check goroutines end")
	close(fakeRepository)
	<-processChan
	close(processChan)
	log.Println("check fake success")
}

func processFakeRepo(processChan chan struct{}, content string) {
	defer func() { processChan <- struct{}{} }()
	fakeNames := make([]string, 0, 10)
	scanner := bufio.NewScanner(strings.NewReader(content))
	linearr := make([]string, 0, len(content)/128)
	for scanner.Scan() {
		linearr = append(linearr, scanner.Text())
	}
	rmIdxs := make([]int, 0, 30)
	for repoName := range fakeRepository {
		log.Println("recv fakeRepo", repoName)
		for idx, str := range linearr {
			if strings.Index(str, repoName) != -1 {
				rmIdxs = append(rmIdxs, idx)
			}
		}
		fakeNames = append(fakeNames, repoName)
	}
	if len(fakeNames) == 0 {
		log.Println("processFakeRepo end without fake repo")
		return
	}
	log.Println("fake repos", fakeNames)
	// process fakeRepo

	for v, idx := range rmIdxs {
		linearr = append(linearr[:idx-v], linearr[idx-v+1:]...)
	}
	createPR(fakeNames, strings.Join(linearr, "\n"))
}

type PrBot struct {
	Commit      string `json:"commit"`
	Description string `json:"description"`
	Files       []struct {
		Content string `json:"content"`
		Path    string `json:"path"`
	} `json:"files"`
	Repo  string `json:"repo"`
	Title string `json:"title"`
	Token string `json:"token"`
	User  string `json:"user"`
}

func createPR(fakeRepo []string, content string) error {
	/**
	curl -X POST \
	  https://xrbhog4g8g.execute-api.eu-west-2.amazonaws.com/prod/prb0t \
	  -H 'cache-control: no-cache' \
	  -H 'content-type: application/json' \
	  -d '{
	  "user": "whtiehack",
	  "repo": "hello-world",
	  "description": "aaa",
	  "title": "Dare to try",
	  "commit": "a try",
	  "files": [
		{"path": "Config.txt", "content": "Failure is when you stop trying to do something."}
	  ],"token":""
	}'
	*/
	const url = "https://xrbhog4g8g.execute-api.eu-west-2.amazonaws.com/prod/prb0t"
	const contentType = "application/json; charset=utf-8"
	prbot := PrBot{}
	prbot.User = "whtiehack"
	prbot.Repo = "996.ICU"
	prbot.Description = "Fake repositories:\n" + githubPrefix[1:] + strings.Join(fakeRepo, "\n"+githubPrefix[1:]) + " "
	prbot.Title = "Remove fake license repositories."
	prbot.Commit = prbot.Title
	prbot.Files = []struct {
		Content string `json:"content"`;
		Path    string `json:"path"`
	}{
		{Content: content, Path: projectPath[1:]},
	}
	prbot.Token = TOKEN
	pb, _ := json.Marshal(prbot)
	fmt.Println("pb", string(pb))
	resp, err := http.Post(url, contentType, bytes.NewReader(pb))
	if err != nil {
		log.Println("createPR error", err)
		return err
	}
	defer resp.Body.Close()
	body, _ := ioutil.ReadAll(resp.Body)
	log.Println("createPR result", string(body))
	return nil
}

func processUrl(c <-chan string, wg *sync.WaitGroup) {
	defer wg.Done()
	count := 0
	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: TOKEN},
	)
	tc := oauth2.NewClient(ctx, ts)
	client := github.NewClient(tc)
	for repoName := range c {
		count++
		log.Println("begin check ", repoName)
		b, err := checkHas996(repoName, client)
		if err != nil {
			log.Println("error", repoName, err)
			continue
		}
		if !b {
			// 假996license仓库
			fakeRepository <- repoName
		}
	}
	log.Println("process end", count)
}

func checkHas996(repo string, client *github.Client) (bool, error) {
	csr, resp, err := client.Search.Code(context.TODO(), "996 repo:"+repo+" in:file", nil)
	if err != nil {
		return false, err
	}
	defer resp.Body.Close()
	if csr == nil || resp.StatusCode != 200 {
		return false, nil
	}
	for _, item := range csr.CodeResults {
		if strings.HasPrefix(strings.ToUpper(*item.Path), "LICENSE") {
			return true, nil
		}
	}
	return false, nil
}

func RemoveDuplicatesAndEmpty(a []string) (ret []string) {
	a_len := len(a)
	for i := 0; i < a_len; i++ {
		if (i > 0 && a[i-1] == a[i]) || len(a[i]) == 0 {
			continue;
		}
		ret = append(ret, a[i])
	}
	return
}
