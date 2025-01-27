import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
    const {siteConfig} = useDocusaurusContext();
    return (
        <header className={clsx('hero hero--primary', styles.heroBanner)}>
            <div className="container">
                <Heading as="h1" className="hero__title">
                    {siteConfig.title}
                </Heading>
                <p className="hero__subtitle">{siteConfig.tagline}</p>
            </div>
        </header>
    );
}

const snippet = `\
-- app.lua

get("/", function()
    return json.encode({ id = 1, name = "Alice", uuid = uuid.v4() })
end)
`;

const shellSnippet = `\
$ redox run app.lua
`;

export default function Home(): ReactNode {
    const {siteConfig} = useDocusaurusContext();
    return (
        <Layout
            title={`${siteConfig.title}`}
            description="Description will go into a meta tag in <head />">
            <HomepageHeader/>
            <main>
                <div className="container"
                     style={{display: "flex", flexDirection: "row", justifyContent: "space-between", padding: 16}}>
                    <div style={{padding: 20}}>
                        <p><b>RE</b>DOX is a high performance Lua runtime built specifically for web microservices.</p>
                        <p>It is built on top of the Rust ecosystem and exposes its wide array of modules over a simple
                            to use LUA api.</p>
                        <p><b>RE</b>DOX and its Rust modules provide one of the most comprehensive and easy to use
                            collection of standard libraries, to keep all this in check it comes in several distros.</p>
                        <pre>{snippet}</pre>
                        <p>Run your app with the <code>redox</code> command.</p>
                        <pre>{shellSnippet}</pre>
                    </div>
                    <img height={400} src={"/img/redox_vs_popular_node_runtimes.svg"}/>
                </div>
            </main>
        </Layout>
    );
}
