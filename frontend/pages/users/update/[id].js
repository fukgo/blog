import { useRouter } from 'next/router';
import { useState, useEffect } from 'react';
import { getUserUrl, updateUserUrl } from '@/api_list';
import ProtectedComponent from "@/components/ProtectedComponent";

const UpdateUser = () => {
    const router = useRouter();
    const { id } = router.query;
    const [user, setUser] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [nickname, setNickname] = useState('');

    useEffect(() => {
        const fetchUser = async () => {
            try {
                const response = await fetch(getUserUrl(id), {
                    credentials: 'include'
                });
                if (!response.ok) throw new Error('无法获取用户信息');
                const userData = await response.json();
                setUser(userData);
                setNickname(userData.nickname);
            } catch (error) {
                setError(error.message);
            } finally {
                setLoading(false);
            }
        };

        if (id) fetchUser();
    }, [id]);

    const handleUpdate = async () => {
        try {
            const response = await fetch(updateUserUrl(id), {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
                body: JSON.stringify({ nickname }),
            });
            if (!response.ok) throw new Error('更新失败');
            router.push(`/users/${id}`);
        } catch (error) {
            setError(error.message);
        }
    };

    if (loading) return <p>加载中...</p>;
    if (error) return <p className="text-red-500">{error}</p>;

    return (
        <ProtectedComponent>
            <div className="container mx-auto p-4">
                <h1 className="text-4xl font-bold mb-4">更新用户信息</h1>
                <div className="bg-white p-6 shadow-lg rounded-lg">
                    <div className="mb-4">
                        <label className="block text-lg mb-2">昵称</label>
                        <input
                            type="text"
                            value={nickname}
                            onChange={(e) => setNickname(e.target.value)}
                            className="w-full p-2 border rounded"
                        />
                    </div>
                    <button
                        onClick={handleUpdate}
                        className="px-4 py-2 bg-blue-500 text-white rounded">
                        更新
                    </button>
                </div>
            </div>
        </ProtectedComponent>
    );
};

export default UpdateUser;