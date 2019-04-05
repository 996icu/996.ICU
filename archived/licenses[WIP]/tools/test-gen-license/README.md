# Test for license generator tools

## The purpose
The 999.ICU project has many(now, the number is 3) license generator tools and it seems that the number of that will continue to grow. Thus I write this test tool to validate the functionality of the tools.

I hope the newly added tools must add a test file and pass the tests.

## Usage 
For example, if you would like to add a license generator tool written by `ruby`, you just need to copy the `test_go.rs` file and rename it as `test_ruby.rs`, and replace the parameter in `arg()` of the test function. 