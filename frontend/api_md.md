需要登录的 API

POST /users                # 创建新用户
PUT /users                 # 更新当前用户信息
DELETE /users/{userId}     # 删除当前用户
DELETE /users/logout         # 用户退出登录

POST /users/{userId}/articles           # 创建文章
POST /users/{userId}/articles/{articleId} # 更新某篇文章
GET /tags/{tagName}            # 创建标签
DELETE /tags/{tagId}    # 删除标签

不需要登录的 API

默认一页为10个文章,default page = 1
GET /users                # 获取所有用户
GET /users/{userId}       # 获取某个用户的信息
GET /users/{userId}/articles?page=1  # 获取某个用户的所有文章标题信息（分页）
GET /users/{userId}/articles/{articleId}  # 获取某篇文章
GET /articles?title={title}&username={username}&page={1} # 根据标题和用户名搜索文章（分页）
GET /tags/all                 # 获取所有标签
GET /tags/{tagId}/articles?page=1  # 获取特定标签下的所有文章（分页）