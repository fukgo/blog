import Head from 'next/head';
import { useState, useEffect } from 'react';
import ArticleList from '../components/ArticleList';
import Modal from '../components/Modal';
import {authTokenUrl, getAllArticleUrl} from "@/api_list";

export default function Home() {
    const [page, setPage] = useState(1); // 当前页码
    const [limit, setLimit] = useState(10); // 每页文章数量
    const [modalMessage, setModalMessage] = useState(null); // 模态框消息
    const [isModalOpen, setIsModalOpen] = useState(false); // 模态框状态

    useEffect(() => {
        // 获取 URL 查询参数
        const urlParams = new URLSearchParams(window.location.search);
        const token = urlParams.get('token');
        console.log(token);

        if (token) {
            // 将 token 存储到 sessionStorage 中
            sessionStorage.setItem('authToken', token);

            // 将 token 添加到 header，向 http://127.0.0.1:8002/auth/token 发送 GET 请求验证 token
            fetch(authTokenUrl(), {
                method: 'GET',
                headers: {
                    'Authorization': `Bearer ${token}`
                },
                credentials: 'include' // 确保发送和接收 cookies
            })
                .then(response => {
                    if (response.status === 200) {
                        console.log('登录成功');
                        setModalMessage('登录成功');
                    } else {
                        console.log('登录失败');
                        setModalMessage('登录失败');
                    }
                    setIsModalOpen(true);
                    setTimeout(() => {
                        setIsModalOpen(false);
                    }, 3000);
                })
                .catch(() => {
                    setModalMessage('登录失败');
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
                <title>我的博客 - 首页</title>
                <meta name="description" content="欢迎来到我的博客，在这里我分享关于 Web 开发、编程等方面的知识。" />
            </Head>

            {/* 头部区域 */}
            <section className="bg-gradient-to-r from-blue-500 to-green-500 py-20 text-center text-white">
                <div className="container mx-auto">
                    <h1 className="text-5xl font-extrabold mb-4">欢迎来到博客</h1>
                    <p className="text-lg">分享关于 Web 开发、编程等方面的知识。</p>
                </div>
            </section>

            {/* 精选文章区域 */}
            <section className="py-12 bg-gray-50">
                <div className="container mx-auto px-4">
                    <h2 className="text-3xl font-bold mb-6 text-center">最近文章</h2>

                    {/* 分页控制和每页项目数 */}
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

                    {/* 文章列表 */}
                    <ArticleList
                        page={page}
                        limit={limit}
                        setPage={setPage}
                        setLimit={setLimit}
                        token={sessionStorage.getItem('authToken')} // 传递 token
                    />


                </div>
            </section>

            {/* 模态框 */}
            {isModalOpen && (
                <div isOpen={isModalOpen} onClose={() => setIsModalOpen(false)}>
                    {modalMessage}
                </div>
            )}
        </div>
    );
}