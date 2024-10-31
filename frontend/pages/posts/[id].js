import { useEffect, useState } from 'react';
import { useRouter } from 'next/router';
import Link from 'next/link';
import { FaUser } from 'react-icons/fa';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { getArticleDetailUrl, getUserUrl, getArticleTagsUrl } from '@/api_list';
import CommentSection from '@/components/CommentSection';
import useAuth from '@/components/useAuth';

const PostDetail = ({ initialArticle, initialUser }) => {
    const router = useRouter();
    const { id } = router.query;

    const [article, setArticle] = useState(initialArticle);
    const [user, setUser] = useState(initialUser);
    const [loading, setLoading] = useState(!initialArticle || !initialUser);
    const [error, setError] = useState(null);
    const [headings, setHeadings] = useState([]);
    const [tags, setTags] = useState([]);
    const [tagLoading, setTagLoading] = useState(true);
    const [isAuthenticated, userLoading] = useAuth();
    const [sessionUser, setSessionUser] = useState(null); // 使用数组解构语法
    const [userDetailId, setUserDetailId] = useState(null); // 用户ID
    useEffect(() => {
        const getSessionUser = async () => {
            if (isAuthenticated) {
                const storedUser = JSON.parse(sessionStorage.getItem('user'));
                if (storedUser && storedUser.user) {
                    setSessionUser(storedUser.user);
                } else {
                    console.error('User information not found');
                }
            }
        };
        getSessionUser();
    }, [isAuthenticated]);

    useEffect(() => {
        const fetchArticleData = async () => {
            if (!id) return;

            try {
                const articleResponse = await fetch(getArticleDetailUrl(id), { credentials: 'include' });
                if (!articleResponse.ok) throw new Error('无法获取文章');
                const articleData = await articleResponse.json();
                setArticle(articleData);

                const userResponse = await fetch(getUserUrl(articleData.user_detail_id), { credentials: 'include' });
                if (!userResponse.ok) throw new Error('无法获取用户信息');
                const userData = await userResponse.json();
                setUser(userData);
            } catch (error) {
                setError(error.message);
            } finally {
                setLoading(false);
            }
        };

        if (id) fetchArticleData();
    }, [id]);

    useEffect(() => {
        const fetchArticleTags = async () => {
            if (!article || !article.id) return;

            try {
                const response = await fetch(getArticleTagsUrl(article.id), { credentials: 'include' });
                if (!response.ok) throw new Error('无法获取标签');
                const tagsData = await response.json();
                setTags(tagsData);
            } catch (error) {
                console.error(error);
            } finally {
                setTagLoading(false);
            }
        };

        if (article && article.id) fetchArticleTags();
    }, [article]);

    if (loading) return <p>加载中...</p>;
    if (error) return <p className="text-red-500">{error}</p>;

    return (
        <div className="container mx-auto p-6 flex flex-col lg:flex-row max-w-7xl">
            {/* Main Article Section */}
            <div className="w-full lg:w-3/4 p-6 bg-white shadow-md rounded-lg">
                <div className="border-b mb-4 pb-4 flex justify-between items-center">
                    <div>
                        <h1 className="text-4xl font-bold text-gray-800 mb-4">{article.title}</h1>
                        <div className="flex items-center text-gray-800">
                            <FaUser className="mr-2 text-gray-600" />
                            <strong>昵称:</strong>
                            <Link href={`/users/${user.id}`} className="ml-2 text-blue-500 hover:underline">
                                {user.nickname}
                            </Link>
                        </div>
                    </div>
                    {/* Edit Button for Author */}
                    {sessionUser && sessionUser.user_detail_id === article.user_detail_id && (
                        <Link href={`/posts/update/${article.id}`}>
                            <button className="bg-blue-500 text-white px-4 py-2 rounded">编辑</button>
                        </Link>
                    )}
                </div>

                {/* Article Tags */}
                <div className="flex flex-wrap gap-2 mt-4 mb-6">
                    {tagLoading ? (
                        <p className="text-gray-500">标签加载中...</p>
                    ) : tags.length > 0 ? (
                        tags.map(tag => (
                            <span
                                key={tag.id}
                                className="bg-blue-100 text-blue-600 px-3 py-1 rounded-full text-sm font-semibold"
                            >
                                #{tag.tag}
                            </span>
                        ))
                    ) : (
                        <p className="text-gray-500">暂无标签</p>
                    )}
                </div>

                {/* Article Digest */}
                <div className="p-4 border-b mb-4 bg-gray-50 text-gray-700">
                    <p>{article.digest}</p>
                </div>

                {/* Featured Badge */}
                {article.feature && (
                    <div className="p-4 mb-4 bg-yellow-100 text-yellow-800 font-semibold rounded">
                        精选文章
                    </div>
                )}

                {/* Article Content */}
                <div className="markdown-body border rounded p-4 bg-gray-50 text-gray-800 mb-6">
                    {article.content ? (
                        <ReactMarkdown remarkPlugins={[remarkGfm]}>{article.content}</ReactMarkdown>
                    ) : (
                        <p className="text-gray-500">此文章没有内容。</p>
                    )}
                </div>

                {/* Comments Section */}
                <div className="border-t pt-6">
                    <h2 className="text-2xl font-semibold mb-4">评论区</h2>
                    <CommentSection articleId={article.id} />
                </div>
            </div>

            {/* Sidebar Navigation */}
            <aside className="w-full lg:w-1/4 lg:pl-6 lg:sticky lg:top-20 mt-8 lg:mt-0">
                <div className="bg-white p-4 shadow-md rounded-lg mb-6">
                    <h2 className="text-lg font-semibold mb-4">文章导航</h2>
                    <ul className="text-gray-600">
                        {headings.length > 0 ? (
                            headings.map(({ text, slug, level }) => (
                                <li key={slug} className={`ml-${(level - 1) * 4}`}>
                                    <a href={`#${slug}`} className="hover:underline">
                                        {text}
                                    </a>
                                </li>
                            ))
                        ) : (
                            <li>暂无导航标题</li>
                        )}
                    </ul>
                </div>
                <Link href="/" className="block text-blue-600 hover:underline text-lg mt-6 text-center">
                    ← 返回文章列表
                </Link>
            </aside>
        </div>
    );
};

export default PostDetail;