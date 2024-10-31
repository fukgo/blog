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
    const [avatar, setAvatar] = useState('');
    const [skills, setSkills] = useState([]);
    const [skillInput, setSkillInput] = useState('');
    const [bio, setBio] = useState('');

    useEffect(() => {
        const fetchUser = async () => {
            try {
                const response = await fetch(getUserUrl(id), {
                    credentials: 'include'
                });
                if (!response.ok) throw new Error('无法获取用户信息');
                const userData = await response.json();
                setUser(userData);
                setNickname(userData.nickname || '');
                setAvatar(userData.avatar || '');
                setSkills(userData.skills ? userData.skills.split(',') : []);
                setBio(userData.bio || '');
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
                body: JSON.stringify({ nickname, avatar, skills: skills.join(','), bio }),
            });
            if (!response.ok) throw new Error('更新失败');
            router.push(`/users/${id}`);
        } catch (error) {
            setError(error.message);
        }
    };

    const addSkill = () => {
        if (skillInput.trim()) {
            setSkills([...skills, skillInput.trim()]);
            setSkillInput('');
        }
    };

    const removeSkill = (skillToRemove) => {
        setSkills(skills.filter(skill => skill !== skillToRemove));
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
                    <div className="mb-4">
                        <label className="block text-lg mb-2">头像 URL</label>
                        <input
                            type="text"
                            value={avatar}
                            onChange={(e) => setAvatar(e.target.value)}
                            className="w-full p-2 border rounded"
                        />
                    </div>
                    <div className="mb-4">
                        <label className="block text-lg mb-2">技能</label>
                        <div className="flex">
                            <input
                                type="text"
                                value={skillInput}
                                onChange={(e) => setSkillInput(e.target.value)}
                                className="w-full p-2 border rounded"
                                placeholder="输入技能并添加"
                            />
                            <button
                                onClick={addSkill}
                                className="ml-2 px-4 py-2 bg-blue-500 text-white rounded"
                            >
                                添加
                            </button>
                        </div>
                        <div className="mt-2 flex flex-wrap gap-2">
                            <span className="text-gray-600">已添加技能: </span>
                            {skills.map((skill, index) => (
                                <span key={index} className="bg-blue-100 text-blue-800 p-2 rounded-full flex items-center">
                                    {skill}
                                    <button
                                        onClick={() => removeSkill(skill)}
                                        className="ml-2 text-red-600 hover:text-red-800"
                                        aria-label={`删除 ${skill}`}
                                    >
                                        &times; {/* X icon */}
                                    </button>
                                </span>
                            ))}
                        </div>
                    </div>
                    <div className="mb-4">
                        <label className="block text-lg mb-2">个人简介</label>
                        <textarea
                            value={bio}
                            onChange={(e) => setBio(e.target.value)}
                            className="w-full p-2 border rounded"
                            rows="4"
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
