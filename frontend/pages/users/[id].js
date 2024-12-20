import { getAuthUserSessionUrl, getUserArticlesUrl, getUserUrl } from '@/api_list';
import { FaCalendarAlt, FaUser } from "react-icons/fa";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import Link from "next/link";
import dayjs from 'dayjs';

const UserProfile = ({ initialUser, initialArticles }) => {
    const router = useRouter();
    const { id } = router.query;
    const [user, setUser] = useState(initialUser);
    const [articles, setArticles] = useState(initialArticles);
    const [loading, setLoading] = useState(!initialUser || !initialArticles);
    const [error, setError] = useState(null);
    const [page, setPage] = useState(1);
    const [limit, setLimit] = useState(10);
    const [totalPages, setTotalPages] = useState(1);
    const [isAuthenticated, setIsAuthenticated] = useState(false);


    useEffect(() => {
        const checkAuth = async () => {
            try {
                const response = await fetch(getAuthUserSessionUrl(), {
                    credentials: 'include'
                });
                if (response.ok) {
                    setIsAuthenticated(true);
                }
            } catch (error) {
                console.error('Authentication check failed:', error);
            }
        };

        checkAuth();
    }, []);

    useEffect(() => {
        const fetchUserData = async () => {
            if (!id) return;

            try {
                const userResponse = await fetch(getUserUrl(id), {
                    credentials: 'include'
                });
                if (!userResponse.ok) throw new Error('无法获取用户信息');
                const userData = await userResponse.json();
                console.log(userData);
                setUser(userData);

                const articlesResponse = await fetch(getUserArticlesUrl(id, page, limit), {
                    credentials: 'include'
                });
                if (!articlesResponse.ok) throw new Error('无法获取用户文章');
                const articlesData = await articlesResponse.json();
                setArticles(articlesData);
                setTotalPages(articlesData[0]?.total_page || 1);
            } catch (error) {
                setError(error.message);
            } finally {
                setLoading(false);
            }
        };

        fetchUserData();
    }, [id, page, limit]);

    if (loading) return <p>加载中...</p>;
    if (error) return <p className="text-red-500">{error}</p>;

    return (
        <div className="container mx-auto p-4">
            <div className="bg-white p-6 shadow-lg rounded-lg mb-8 flex flex-col md:flex-row items-center">
                {/* 用户头像 */}
                <div className="w-32 h-20 md:w-40 md:h-24 rounded-full bg-gray-200 overflow-hidden mb-4 md:mb-0">
                    <img
                        src={`${user.avatar}`} // 可以将用户头像的URL放置在这里
                        alt="avatar"
                        className="w-full h-full object-cover"
                    />
                </div>


                {/* 用户信息 */}
                <div className="md:ml-6 text-center md:text-left">
                    {user.nickname && (
                        <p className="text-lg mb-2 flex items-center">
                            <FaUser className="mr-2"/> 昵称: {user.nickname}
                        </p>
                    )}
                    <p className="text-lg flex items-center">
                        <FaCalendarAlt className="mr-2"/> 创建时间: {dayjs(user.created_at).format('YYYY年M月D日')}
                    </p>
                    {user.skills && (
                        <div className="mt-2">
                            <strong>技能:</strong>
                            <div className="flex flex-wrap mt-1">
                                {user.skills.split(',').map((skill, index) => (
                                    <span key={index}
                                          className="bg-blue-200 text-blue-800 px-3 py-1 rounded-full mr-2 mb-2">
                                {skill.trim()}
                            </span>
                                ))}
                            </div>
                        </div>
                    )}
                    {user.bio && <p className="text-lg mt-2">简介: {user.bio}</p>}


                    {isAuthenticated && (
                        <button
                            onClick={() => router.push(`/users/update/${id}`)}
                            className="mt-4 px-4 py-2 bg-blue-500 text-white rounded">
                            更新用户信息
                        </button>
                    )}
                </div>
            </div>

            {/* 文章列表 */}
            <div>
                <h2 className="text-3xl font-bold mb-6">已发布的文章</h2>
                <div className="flex justify-center mb-6">
                    <label className="mr-4 text-lg">
                    每页项目数:
                        <select
                            value={limit}
                            onChange={(e) => setLimit(Number(e.target.value))}
                            className="ml-2 p-2 bg-white border rounded shadow">
                            <option value={5}>5</option>
                            <option value={10}>10</option>
                            <option value={20}>20</option>
                            <option value={50}>50</option>
                        </select>
                    </label>
                </div>
                {articles.length > 0 ? (
                    <ul className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {articles.map((article) => (
                            <li
                                key={article.id}
                                className="bg-white p-6 rounded-lg shadow-md transition transform hover:scale-105 hover:shadow-lg"
                            >
                                <Link href={`/posts/${article.id}`}>
                                    <h3 className="text-2xl font-semibold text-blue-600 hover:underline">
                                        {article.title}
                                    </h3>
                                    <h6 className="mt-4 text-gray-700">
                                        {article.digest}
                                    </h6>
                                    <p className="text-sm text-gray-500 mt-2">
                                        <strong>发布时间:</strong> {dayjs(article.created_at).format('YYYY年M月D日')}
                                    </p>
                                    <p className="text-sm text-gray-500">
                                        <strong>更新时间:</strong> {dayjs(article.updated_at).format('YYYY年M月D日')}
                                    </p>
                                    {article.tags && article.tags.length > 0 && (
                                        <div className="mt-2">
                                            <strong>标签:</strong> {article.tags.join(', ')}
                                        </div>
                                    )}
                                </Link>
                            </li>
                        ))}
                    </ul>
                ) : (
                    <p>该用户尚未发布任何文章。</p>
                )}
                <div className="flex justify-center mt-6">
                    <button
                        onClick={() => setPage((prev) => Math.max(prev - 1, 1))}
                        disabled={page === 1}
                        className={`px-4 py-2 mr-2 ${page === 1 ? 'bg-gray-300' : 'bg-blue-500 text-white'} rounded`}>
                        上一页
                    </button>
                    <button
                        onClick={() => setPage((prev) => prev + 1)}
                        disabled={page === totalPages}
                        className={`px-4 py-2 ${page === totalPages ? 'bg-gray-300' : 'bg-blue-500 text-white'} rounded`}>
                        下一页
                    </button>
                </div>
                <p className="text-center mt-2">当前页: {page} / {totalPages}</p>
            </div>
        </div>
    );
};

UserProfile.getInitialProps = async ({ query }) => {
    const { id } = query;

    try {
        const userResponse = await fetch(getUserUrl(id), {
            credentials: 'include'
        });
        if (!userResponse.ok) throw new Error('无法获取用户信息');
        const userData = await userResponse.json();

        const articlesResponse = await fetch(getUserArticlesUrl(id, 1, 10), {
            credentials: 'include'
        });
        if (!articlesResponse.ok) throw new Error('无法获取用户文章');
        const articlesData = await articlesResponse.json();

        return { initialUser: userData, initialArticles: articlesData };
    } catch (error) {
        return { initialUser: null, initialArticles: [] };
    }
};

export default UserProfile;