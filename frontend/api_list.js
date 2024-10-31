export const apiDomain = 'http://localhost:8002/api';
export const authDomain = 'http://localhost:8001';
export const localDomain = 'http://localhost:3000';

export const loginUrl = () => `${authDomain}/auth/login?redirect=${localDomain}`;
export const RegisterUrl = () => `${authDomain}/auth/register`;
// export const authTokenUrl = () => `${authDomain}/`;



///articles
export const updateArticleUrl = (articleId) => `${apiDomain}/articles/${articleId}`;
export const createArticleUrl = () => `${apiDomain}/articles`;
export const getArticleDetailUrl = (articleId) => `${apiDomain}/articles/detail/${articleId}`;
export const getFeatureArticleUrl = () => `${apiDomain}/articles/feature`;
export const getLateArticlesUrl = () => `${apiDomain}/articles/late`;
export const getArticlesTitlesUrl = () => `${apiDomain}/articles/titles/all`;
export const getArticleTagsUrl = (articleId) => `${apiDomain}/articles/${articleId}/tags`;
///tags
export const createTagUrl = (tagName) => `${apiDomain}/tags/${tagName}`;
export const deleteTagUrl = (tagId) => `${apiDomain}/tags/delete/${tagId}`;
export const getTagArticlesUrl = (tagId) => `${apiDomain}/tags/${tagId}/articles`;
export const getAllTagsUrl = () => `${apiDomain}/tags/all`;

///users
export const logoutUrl = () => `${apiDomain}/users/logout`;
export const getAllUsersUrl = () => `${apiDomain}/users`;
export const getUserUrl = (userId) => `${apiDomain}/users/${userId}`;
export const getUserArticlesUrl = (userId, page ,limit) => `${apiDomain}/users/${userId}/articles?page=${page}&limit=${limit}`;
export const getUserResume = (userId) => `${apiDomain}/users/${userId}/resume`;
export const updateUserUrl = (userId) => `${apiDomain}/users/${userId}/update`;

//更新或创建用户简历
export const postUserResume = (userId) => `${apiDomain}/users/${userId}/resume`;
export const getAuthUserSessionUrl = () => `${apiDomain}/auth/session`;


///auth
export const authTokenUrl = () => `${apiDomain}/auth/token`;

///catalogues
//post 创建
export const postCatalogueUrl = () => `${apiDomain}/catalogues`;
//post 更新
export const postUpdateCatalogueUrl = (catalogueId) => `${apiDomain}/catalogues/${catalogueId}`;
//get 单个目录信息
export const getCatalogueUrl = (catalogueId) => `${apiDomain}/catalogues/${catalogueId}`;
//get 获取所有的目录以及文章信息
export const getCataloguesArticlesUrl = (catalogueId) => `${apiDomain}/catalogues/${catalogueId}/articles`;
//delete 删除目录
export const deleteCatalogueUrl = (catalogueId) => `${apiDomain}/catalogues/${catalogueId}`;
//get 获取所有目录
export const getCataloguesAllUrl = () => `${apiDomain}/catalogues/all`;
//delete 移除目录下的文章
export const deleteCatalogueArticleUrl = (catalogueId,articleId) => `${apiDomain}/catalogues/delete/${catalogueId}/${articleId}`;

//delete 移除目录下的所有文章
export const deleteCatalogueArticlesUrl = (catalogueId) => `${apiDomain}/catalogues/delete/${catalogueId}/all`;

//post 添加文章到目录
export const postCatalogueArticleUrl = () => `${apiDomain}/catalogues/add`;
//post 更新目录下文章的排序
export const postCatalogueArticleSortUrl = () => `${apiDomain}/catalogues/post/sorder`;
///comments
export const getArticleCommentUrl = (article_id) => `${apiDomain}/comments/${article_id}`;
export const postArticleCommentUrl = () => `${apiDomain}/comments/post`;


