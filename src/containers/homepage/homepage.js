import React from 'react';
import { Helmet } from 'react-helmet';

import { Alert, CardColumns, Container } from 'react-bootstrap';

import Button from 'components/button/button';
import Projects from 'content/projects';
import Featured from 'content/featured';
import ProjectSmall from 'components/projectSmall/projectSmall';

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
                    <div>
                        <Button variant='outline-primary' to='/projects'>Projects</Button>
                        <Button variant='outline-primary' to='/contact'>Contact</Button>
                    </div>
                </Alert>
            </div>
            <Container className="mt-3">
                <div className="d-flex justify-content-between my-2">
                    <h4>Featured Projects</h4>
                    <Button variant="link" to='/projects'>View All Projects</Button>
                </div>
                <CardColumns>
                    {Featured.map(id => Projects.find(project => project.id === id)).map((project, index) => <ProjectSmall
                        key={project.id}
                        history={this.props.history}
                        delay={index}
                        {...project}
                    />)}
                </CardColumns>
            </Container>
        </>;
    }
}