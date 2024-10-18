// useAuth.js
import { useState, useEffect } from 'react';
import axios from 'axios';
import {getAuthUserSessionUrl} from "@/api_list";

const useAuth = () => {
    const [isAuthenticated, setIsAuthenticated] = useState(null); // 初始状态为 null
    const [loading, setLoading] = useState(true); // 加载状态，初始为 true

    useEffect(() => {
        const checkAuth = async () => {
            try {
                const response = await axios.get(getAuthUserSessionUrl(),{
                    withCredentials: true
                });
                if (response.status === 200) {
                    setIsAuthenticated(true); // 用户已登录
                } else {
                    setIsAuthenticated(false); // 用户未登录
                }
            } catch (error) {
                setIsAuthenticated(false); // 请求失败，视为未登录
            } finally {
                setLoading(false); // 请求结束，更新加载状态
            }
        };

        checkAuth();
    }, []);

    return { isAuthenticated, loading };
};

export default useAuth;
