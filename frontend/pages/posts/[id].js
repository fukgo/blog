import { useEffect, useState } from 'react';
import { useRouter } from 'next/router';
import { marked } from 'marked';
import Link from 'next/link';
import { FaUser, FaEnvelope } from 'react-icons/fa';
import hljs from 'highlight.js'; // 导入代码高亮库
import 'highlight.js/styles/github.css'; // 高亮样式
import 'github-markdown-css'; // GitHub Markdown 样式
import '@/api_list'
import {getArticleDetailUrl, getUserUrl} from "@/api_list";

const PostDetail = ({ initialArticle, initialUser }) => {
    const router = useRouter();
    const { id } = router.query;

    const [article, setArticle] = useState(initialArticle);
    const [user, setUser] = useState(initialUser);
    const [loading, setLoading] = useState(!initialArticle || !initialUser);
    const [error, setError] = useState(null);

    useEffect(() => {
        // 配置 marked 以支持 GitHub 风格的 Markdown 和代码高亮
        marked.setOptions({
            gfm: true,
            breaks: true,
            highlight: (code, lang) => {
                const language = lang || 'plaintext'; // 默认语言为 plaintext
                return hljs.highlight(language, code).value; // 返回高亮后的代码
            },
        });

        const fetchArticle = async () => {
            if (!id) return;

            try {
                const articleResponse = await fetch(getArticleDetailUrl(id), {
                    credentials: 'include',
                });
                if (!articleResponse.ok) throw new Error('Unable to fetch article information');
                const articleData = await articleResponse.json();
                setArticle(articleData);

                const userResponse = await fetch(getUserUrl(articleData.user_id), {
                    credentials: 'include',
                });
                if (!userResponse.ok) throw new Error('Unable to fetch user information');
                const userData = await userResponse.json();
                setUser(userData);
            } catch (error) {
                setError(error.message); // 捕获并设置错误信息
            } finally {
                setLoading(false); // 完成加载
            }
        };

        // 仅在 id 存在时请求数据
        if (id) {
            fetchArticle();
        }
    }, [id]); // 依赖 id

    // 加载状态和错误处理
    if (loading) return <p>Loading...</p>;
    if (error) return <p className="text-red-500">{error}</p>;

    return (
        <div className="container mx-auto p-6 max-w-4xl">
            <div className="bg-white shadow-md rounded-lg overflow-hidden">
                {/* 文章标题和作者信息 */}
                <div className="p-6 border-b flex justify-between items-start">
                    <div>
                        <h1 className="text-4xl font-bold text-gray-800 mb-2">{article.title}</h1>
                        <h3 className="text-gray-600 text-sm mb-4 italic">{article.digest || 'No digest available'}</h3>
                    </div>
                    <div className="ml-6 text-right">
                        <h3 className="text-2xl font-semibold mb-4">Author Information</h3>
                        <div className="flex items-center mb-2">
                            <FaUser className="mr-2 text-gray-600" />
                            <p className="text-gray-800">
                                <strong>Username:</strong> {user.username}
                            </p>
                        </div>
                        <div className="flex items-center">
                            <FaEnvelope className="mr-2 text-gray-600" />
                            <p className="text-gray-800">
                                <strong>Email:</strong> {user.email}
                            </p>
                        </div>
                    </div>
                </div>

                {/* 文章内容 */}
                <div className="p-6 bg-gray-50"> {/* 设置浅色背景 */}
                    {article.content ? (
                        <div className="markdown-body prose prose-lg max-w-none" dangerouslySetInnerHTML={{ __html: marked(article.content) }} />
                    ) : (
                        <p className="text-gray-500">This article has no content.</p>
                    )}
                </div>
            </div>

            {/* 返回文章列表按钮 */}
            <Link href="/" className="mt-6 inline-block text-blue-600 hover:underline text-lg">
                ← Back to Posts
            </Link>
        </div>
    );
};

// 获取初始数据
PostDetail.getInitialProps = async ({ query }) => {
    const { id } = query;

    try {
        const articleResponse = await fetch(getArticleDetailUrl(id), {
            credentials: 'include',
        });
        if (!articleResponse.ok) throw new Error('Unable to fetch article information');
        const articleData = await articleResponse.json();

        const userResponse = await fetch(getUserUrl(articleData.user_id), {
            credentials: 'include',
        });
        if (!userResponse.ok) throw new Error('Unable to fetch user information');
        const userData = await userResponse.json();

        return { initialArticle: articleData, initialUser: userData };
    } catch (error) {
        return { initialArticle: null, initialUser: null }; // 出现错误时返回 null
    }
};

export default PostDetail;