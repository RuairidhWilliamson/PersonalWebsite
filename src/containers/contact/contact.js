import React from 'react';
import { Helmet } from 'react-helmet';
import Iframe from 'components/iframe/iframe';
import { ResponsiveEmbed } from 'react-bootstrap';


export default class Contact extends React.Component {
    render() {
        return <>
            <Helmet>
                <title>Contact | Ruairidh Williamson</title>
                <meta name='description' content=''/>
            </Helmet>
            <ResponsiveEmbed>
                <Iframe
                    src="https://docs.google.com/forms/d/e/1FAIpQLSdDZ1cC3dVc_QlJibT4GlpTJybziQ9yjZiBl1TpoQ01jP7CNQ/viewform?embedded=true"
                    frameBorder="0"
                    marginHeight="0"
                    marginWidth="0"
                >
                Loading…
                </Iframe>
            </ResponsiveEmbed>
        </>;
    }
}