#### 文章相关 API    /articles

- POST /articles/ 创建一篇文章
- POST /articles/:article_id 更新指定 ID 的文章
- GET  /articles/feature获取随机精选6文章
- GET  /articles/late?page={}?limit={} 获取全部最新文章
- DELETE /articles/:article_id 删除指定 ID 的文章
- GET /detail/:article_id   查询文章详情

#### 标签相关 API    /tags

- POST /tags/  创建一个标签
- DELETE /tags/delete/:tag_id 删除指定 ID 的标签
- GET /tags/all 获取所有标签
- GET /tags/:tag_id/articles?page={}?limit={} 获取指定标签下的所有文章信息

#### 用户相关 API    /users

- GET /users 获取所有用户
- GET  /users/:user_id 获取指定 ID 的用户
- DELETE  /users/logout 用户登出
- DELETE /users/:user_id 删除指定 ID 的用户
- GET /users/:user_id/articles?page={}?limit={} 获取指定用户的所有文章详细
- GET /users/:user_id/resume获取用户resume
- POST /users/:user_id/resume更新或上传resume

#### 认证相关API     /auth

- GET /auth/token    验证token
- GET /auth/session    验证session登录

#### 目录相关API /catalogues

- POST /catalogues    上传catalogue

- GET/catalogues/:catalogue_id    更新catalogue

- DELETE /catalogues/:catalogue_id    删除

- GET /catalogues/:catalogue_id  获取单个catalogue信息、

- GET /catalogues/:catalogue_id/articles 获取所有的目录以及文章信息



#### 评论相关API /comments

- POST/comments/post      上传评论 

- GET/comments/:article_id    获取文章所有评论（递归）

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