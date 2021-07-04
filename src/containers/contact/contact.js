import React from 'react';
import { Helmet } from 'react-helmet';
import Iframe from 'components/iframe/iframe';
import { Container, Jumbotron } from 'react-bootstrap';
import Button from 'components/button/button';
import Icon from 'components/materialIcon/materialIcon';


export default class Contact extends React.Component {
    render() {
        return <>
            <Helmet>
                <title>Contact | Ruairidh Williamson</title>
                <meta name='description' content=''/>
            </Helmet>
            <Jumbotron>
                <Container>
                    <h3>Contact Me</h3>
                    <Button href='mailto:contact@rtaw.co.uk' className='d-inline-flex justify-content-center align-content-between'><Icon className='mr-1' icon='mail'/><span>Email: contact@rtaw.co.uk</span></Button>
                    <h5>
                        Or the form below
                    </h5>
                </Container>
            </Jumbotron>
            <Container>
                <Iframe
                    src="https://docs.google.com/forms/d/e/1FAIpQLSdDZ1cC3dVc_QlJibT4GlpTJybziQ9yjZiBl1TpoQ01jP7CNQ/viewform?embedded=true"
                    width="100%"
                    frameBorder="0"
                    marginHeight="0"
                    marginWidth="0"
                ></Iframe>
            </Container>
        </>;
    }
}