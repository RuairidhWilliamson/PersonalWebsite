import React from 'react';
import { Helmet } from 'react-helmet';

import { Alert } from 'react-bootstrap';

import Button from 'components/button/button';

import './homepage.scss';

export default class Homepage extends React.Component {
    render() {
        return <>
            <Helmet>
                <title>Ruairidh Williamson</title>
                <meta name='description' content=''/>
            </Helmet>
            <div className='homepage'>
                <Alert variant='primary'>
                    <Alert.Heading>Ruairidh Williamson</Alert.Heading>
                    <p>I am currently looking for a summer internship for 2020.</p>
                    <hr/>
                    <div>
                        <Button variant='outline-primary' to='/projects'>Projects</Button>
                        <Button variant='outline-primary' to='/contact'>Contact</Button>
                    </div>
                </Alert>
            </div>
        </>;
    }
}