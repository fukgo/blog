// useAuth.js
import { useState, useEffect } from 'react';

const useAuth = () => {
    const [isAuthenticated, setIsAuthenticated] = useState(false); // 初始状态为 false
    const [loading, setLoading] = useState(true); // 加载状态，初始为 true

    useEffect(() => {
        const checkAuth = async () => {
            try {
                // 从 sessionStorage 获取用户信息
                const userData = JSON.parse(sessionStorage.getItem('user'));
                console.log('检查用户数据:', userData);

                // 检查用户数据是否存在
                if (userData) {
                    // 检查用户数据是否过期
                    if (userData.expiry > Date.now()) {
                        setIsAuthenticated(true); // 用户已登录
                        console.log('用户已登录');
                    } else {
                        console.warn('用户数据已过期');
                        setIsAuthenticated(false); // 用户未登录
                        sessionStorage.removeItem('user'); // 移除过期数据
                    }
                } else {
                    console.warn('用户数据不存在');
                    setIsAuthenticated(false); // 用户未登录
                }
            } catch (error) {
                console.error('检查认证时发生错误:', error);
                setIsAuthenticated(false); // 请求失败，视为未登录
            } finally {
                setLoading(false); // 请求结束，更新加载状态
                console.log('加载状态更新为 false');
            }
        };

        checkAuth();
    }, []);

    return [isAuthenticated, loading];
};

export default useAuth;