// Copyright © 2019 Andy Pan <panjf2000@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// The source repository URL: https://github.com/panjf2000/gen-license-go

package cmd

import (
	"os"
	"fmt"
	"path"
	"strings"
	"io/ioutil"

	"github.com/spf13/cobra"
)

var LICENSES []string

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "gen-license-go",
	Short: "gen-license-go is a open-source licenses generator implemented in Go.",
	Long: `gen-license-go is a 996.icu license generator implemented in Go,
this generator is developed to generate various open-source licenses including MIT, Apache, etc.
More importantly, the main purpose of this tool is to incorporate those aforesaid licenses into
a brand new license: 996.icu, defined by this repository.`,
	Run: func(cmd *cobra.Command, args []string) {
		for _, license := range LICENSES {
			fmt.Println(license)
		}
	 },
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func init() {
	// Read all filenames from licenses directory.
	files, err := ioutil.ReadDir("./licenses")
	handleError(err)
	for _, file := range files {
			if file.IsDir() {
					continue
			} else {
					LICENSES = append(LICENSES, strings.TrimSuffix(file.Name(), path.Ext(file.Name())))
			}
	}

	// Here you will define your flags and configuration settings.
	// Cobra supports persistent flags, which, if defined here,
	// will be global for your application.
	// rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.genLicense.yaml)")

	// Cobra also supports local flags, which will only run
	// when this action is called directly.
	// rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	rootCmd.Flags().BoolP("list", "l", true, "list all licenses")
	rootCmd.MarkFlagRequired("list")
}
