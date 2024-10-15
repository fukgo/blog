// pages/tags/[id].js
import { useRouter } from 'next/router';
import ArticleList from '../../components/ArticleList';
import { useState } from 'react';

const TagPage = () => {
    const router = useRouter();
    const { id } = router.query; // 从 URL 获取标签 ID
    const [page, setPage] = useState(1); // 当前页码
    const [limit, setLimit] = useState(10); // 每页文章数量

    if (!id) return <p>加载中...</p>;

    return (
        <div>
            <h1 className="text-3xl font-bold mb-6 text-center">此标签的文章</h1>

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
                url={`http://127.0.0.1:8002/tags/${id}/articles`}
                page={page}
                limit={limit}
                setPage={setPage}
                setLimit={setLimit}
            />

            {/* <div className="flex justify-center mt-6">
                <button
                    onClick={() => setPage((prev) => Math.max(prev - 1, 1))}
                    disabled={page === 1}
                    className={`px-4 py-2 mr-2 ${page === 1 ? 'bg-gray-300' : 'bg-blue-500 text-white'} rounded`}>
                    上一页
                </button>
                <button
                    onClick={() => setPage((prev) => prev + 1)}
                    className="px-4 py-2 bg-blue-500 text-white rounded">
                    下一页
                </button>
            </div>

            <p className="text-center mt-2">当前页: {page}</p> */}
        </div>
    );
};

export default TagPage;