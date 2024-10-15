// pages/about.js
import Link from 'next/link';
import { useTranslation } from 'react-i18next';

const About = () => {
    const { t } = useTranslation();

    return (
        <div className="container mx-auto p-6 max-w-4xl">
            <div className="bg-white shadow-lg rounded-lg overflow-hidden">
                <div className="p-6 border-b">
                    <h1 className="text-4xl font-bold text-gray-800 mb-2">{t('about.title', { defaultValue: '关于我们' })}</h1>
                </div>
                <div className="p-6 bg-gray-50 markdown-body prose prose-lg max-w-none">
                    <p>{t('about.welcome', { defaultValue: '欢迎来到我们的博客平台！' })}</p>
                    <p>{t('about.techStack', { defaultValue: '我们使用 Rust 和 JavaScript 构建了这个分布式微服务博客。' })}</p>
                    <p>{t('about.backend', { defaultValue: '后端框架' })}: <strong>Axum</strong></p>
                    <p>{t('about.frontend', { defaultValue: '前端框架' })}: <strong>Next.js</strong></p>
                    <p>{t('about.goal', { defaultValue: '我们的目标是搭建一个认证分离的分布式微服务博客。' })}</p>
                    <p>
                        {t('about.projectLink', { defaultValue: '项目地址' })}: 
                        <a href="https://github.com/fukgo/blog" className="text-blue-600 hover:underline">GitHub</a>
                    </p>
                    <p>{t('about.joinUs', { defaultValue: '我们希望更多的人能够加入我们，一起构建这个博客平台。' })}</p>
                </div>
                <div className="p-6 flex flex-col items-center">
                    <Link href="/" className="mt-6 inline-block bg-blue-600 text-white rounded-lg px-4 py-2 hover:bg-blue-700">
                        ← {t('about.backToHome', { defaultValue: '返回首页' })}
                    </Link>
                    <div className="mt-4">
                        <a href="https://github.com/fukgo/blog" className="text-blue-600 hover:underline">
                            {t('about.joinDevelopers', { defaultValue: '加入开发者' })}
                        </a>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default About;
