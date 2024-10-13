// pages/tags/[id].js
import { useRouter } from 'next/router';
import ArticleList from '../../components/ArticleList';
import { useState } from 'react';

const TagPage = () => {
    const router = useRouter();
    const { id } = router.query; // Get tag ID from the URL
    const [page, setPage] = useState(1); // Current page
    const [limit, setLimit] = useState(10); // Number of articles per page

    if (!id) return <p>Loading...</p>;

    return (
        <div>
            <h1 className="text-3xl font-bold mb-6 text-center">Articles for Tag </h1>

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
                url={`http://127.0.0.1:8002/tags/${id}/articles`}
                page={page}
                limit={limit}
                setPage={setPage}
                setLimit={setLimit}
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
    );
};

export default TagPage;