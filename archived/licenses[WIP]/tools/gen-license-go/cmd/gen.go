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
	"fmt"
	"strings"
	"errors"
	"io/ioutil"

	"github.com/spf13/cobra"
)

var template string
var licensePathTemplate, icuPathTemplate = "licenses/%s.txt", "licenses/templates/996.icu.template.%s.txt"

// genCmd represents the gen command
var genCmd = &cobra.Command{
	Use:   "gen",
	Short: "gen is a 996.icu license generator-command.",
	Long: `gen is a 996.icu license generator-command,
it is used to generate various open-source licenses including MIT, Apache, etc.
More importantly, the main purpose of this tool is to incorporate those aforesaid licenses into
a brand new license: 996.icu, defined by this repository.

How to use gen:
# Generate a pure open-source license, take MIT for example:
gen-license-go mit

# Incorporate a open-source(MIT) license into the 996icu license with a specific-language(en-us or zh-cn) template:
gen-license-go gen mit --996icu en-us`,
	Args: func(cmd *cobra.Command, args []string) error {
		if len(args) < 1 {
			return errors.New("missing license name to generate 996icu license")
		}
		if ok := isValidLicense(args[0]); !ok {
			fmt.Printf("Invalid license type: %s, supported licenses:\n**********************************************\n", args[0])
			for _, v := range LICENSES {
				fmt.Println(v)
			}
			fmt.Println("**********************************************")
			return errors.New("please check your input")
		}
		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		licenseContent, err := ioutil.ReadFile(fmt.Sprintf(licensePathTemplate, args[0]))
		handleError(err)
		if template != "" {
			icuTemplate, err := ioutil.ReadFile(fmt.Sprintf(icuPathTemplate, template))
			handleError(err)
			newLicenseContent := strings.Replace(strings.Replace(string(icuTemplate), "{other}", string(args[0]), -1), "{content}", string(licenseContent), 1)
			handleError(ioutil.WriteFile("LICENSE", []byte(newLicenseContent), 0644))
		} else {
			handleError(ioutil.WriteFile("LICENSE", licenseContent, 0644))
		}
	},
}

func init() {
	// Add the 'gen' sub-command into root-command.
	rootCmd.AddCommand(genCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// genCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// genCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
	genCmd.Flags().StringVar(&template, "996icu", "", "incorporate a specific license into 996icu license")
}

func handleError(e error) {
	if e != nil {
		panic(e)
	}
}

func isValidLicense(license string) bool {
	for _, v := range LICENSES {
		if license == v {
			return true
		}
	}
	return false
}
