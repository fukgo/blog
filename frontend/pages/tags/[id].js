// pages/tags/[id].js
import { useRouter } from 'next/router';
import ArticleList from '../../components/ArticleList';
import { useState } from 'react';
import {getTagArticlesUrl} from "@/api_list";
import TagArticleList from "@/components/TagArticleList";

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
            <TagArticleList
                id={id}
                page={page}
                limit={limit}
                setPage={setPage}
                setLimit={setLimit}
            />


        </div>
    );
};

export default TagPage;