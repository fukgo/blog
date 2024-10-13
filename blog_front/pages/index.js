import Head from 'next/head';
import { useState, useEffect } from 'react';
import ArticleList from '../components/ArticleList';
import Modal from '../components/Modal';

export default function Home() {
    const [page, setPage] = useState(1); // 当前页码
    const [limit, setLimit] = useState(10); // 每页文章数量
    const [modalMessage, setModalMessage] = useState(null); // Modal message
    const [isModalOpen, setIsModalOpen] = useState(false); // Modal state

    useEffect(() => {
        // 获取 URL 查询参数
        const urlParams = new URLSearchParams(window.location.search);
        const token = urlParams.get('token');
        console.log(token);

        if (token) {
            // 将 token 存储到 sessionStorage 中
            sessionStorage.setItem('authToken', token);

            // 将 token 添加到 header，向 http://127.0.0.1:8002/auth/token 发送 GET 请求验证 token
            fetch('http://127.0.0.1:8002/auth/token', {
                method: 'GET',
                headers: {
                    'Authorization': `Bearer ${token}`
                },
                credentials: 'include' // This ensures cookies are sent and received
            })
                .then(response => {
                    if (response.status === 200) {
                        console.log('Login successful');
                        setModalMessage('Login successful');
                    } else {
                        console.log('Login failed');
                        setModalMessage('Login failed');
                    }
                    setIsModalOpen(true);
                    setTimeout(() => {
                        setIsModalOpen(false);
                    }, 3000);
                })
                .catch(() => {
                    setModalMessage('Login failed');
                    setIsModalOpen(true);
                    setTimeout(() => {
                        setIsModalOpen(false);
                    }, 3000);
                });
        }
    }, []); // 仅在组件挂载时运行

    return (
        <div>
            <Head>
                <title>My Blog - Home</title>
                <meta name="description" content="Welcome to my blog where I share knowledge about web development, programming, and more." />
            </Head>

            {/* Hero Section */}
            <section className="bg-gradient-to-r from-blue-500 to-green-500 py-20 text-center text-white">
                <div className="container mx-auto">
                    <h1 className="text-5xl font-extrabold mb-4">Welcome to My Blog</h1>
                    <p className="text-lg">Sharing knowledge about web development, programming, and more.</p>
                </div>
            </section>

            {/* Featured Posts Section */}
            <section className="py-12 bg-gray-50">
                <div className="container mx-auto px-4">
                    <h2 className="text-3xl font-bold mb-6 text-center">Featured Posts</h2>

                    {/* Pagination controls and items per page */}
                    <div className="flex justify-center mb-6">
                        <label className="mr-4 text-lg">
                            Items per page:
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

                    {/* Article List */}
                    <ArticleList
                        url="http://127.0.0.1:8002/articles/all"
                        page={page}
                        limit={limit}
                        setPage={setPage}
                        setLimit={setLimit}
                        token={sessionStorage.getItem('authToken')} // 传递 token
                    />

                    {/* Pagination buttons */}
                    <div className="flex justify-center mt-6">
                        <button
                            onClick={() => setPage((prev) => Math.max(prev - 1, 1))}
                            disabled={page === 1}
                            className={`px-4 py-2 mr-2 ${page === 1 ? 'bg-gray-300' : 'bg-blue-500 text-white'} rounded`}>
                            Previous
                        </button>
                        <button
                            onClick={() => setPage((prev) => prev + 1)}
                            className="px-4 py-2 bg-blue-500 text-white rounded">
                            Next
                        </button>
                    </div>

                    {/* Current page info */}
                    <p className="text-center mt-2">Current page: {page}</p>
                </div>
            </section>

            {/* Modal */}
            {isModalOpen && (
                <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)}>
                    {modalMessage}
                </Modal>
            )}
        </div>
    );
}