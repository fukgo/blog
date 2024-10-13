// pages/_app.js
import "@/styles/globals.css"; // 保留一个全局样式引入
import Layout from "@/components/Layout"; // 引入Layout组件，使用相对路径
export default function App({ Component, pageProps }) {
    return (
        <Layout>
            <Component {...pageProps} />
        </Layout>
    );
}
