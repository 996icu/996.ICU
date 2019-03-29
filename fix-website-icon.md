# 修复网站图标不展示 #
## https://996.icu网站图标未显示出来 ##
### 示例代码
    
	<!DOCTYPE html>
	<html lang="en">
	<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>996.ICU</title>
    <link rel="shortcut icon" type="image/x-icon" href="https://avatars2.githubusercontent.com/u/48942249?s=60&v=4"/>
	</head>
	<body>
    
	</body>
	</html>
#### 可能是图片未上传至网站域名下导致
    
	<link rel="shortcut icon" type="image/x-icon" href="https://avatars2.githubusercontent.com/u/48942249?s=60&v=4"/>	
#### 网址图标代码替换为这句代码或者换图片地址应该也可以