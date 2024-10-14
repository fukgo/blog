#### 文章相关 API	/articles

- POST /articles/ 创建一篇文章
- POST /articles/:article_id 更新指定 ID 的文章
- GET  /articles/all?page={}?limit={} 获取所有文章
- DELETE /articles/:article_id 删除指定 ID 的文章
- GET /detail/:article_id   查询文章详情

#### 标签相关 API	/tag

- POST /tags/  创建一个标签
- DELETE /tags/:tag_id 删除指定 ID 的标签
- GET /tags/all 获取所有标签
- GET /tags/:tag_id/articles?page={}?limit={} 获取指定标签下的所有文章信息

#### 用户相关 API	/users

- GET /users 获取所有用户
- GET  /users/:user_id 获取指定 ID 的用户
- DELETE  /users/logout 用户登出
- DELETE /users/:user_id 删除指定 ID 的用户
- GET /users/:user_id/articles?page={}?limit={} 获取指定用户的所有文章详细

#### 认证相关API 	/auth

- GET /auth/token	验证token
- GET /auth/session    验证session登录

*除GET请求外都需要验证登录*

auth模块

POST /login

```json
{
    "username",
	"password"
}
```



POST /register

```json
{
    "username",
    "email",
	"password"
}
```

GET host/

验证请求头token