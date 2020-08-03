import React from 'react';
import PropTypes from 'prop-types';

import Button from 'components/button/button';
import Projects from 'content/projects';
import GitHub from 'assets/images/github32.png';
import './projectPage.scss';

const nl2br = (text) => text.replace(/ *\r?\n/g, '\n').split('\n\n').map((item, key) => <span key={key}>{item}<br/></span>);

export default class ProjectPage extends React.Component {

    renderPage(page) {
        return <div className='project-page'>
            <div className='project'>
                <h1 className='title'>{page.title}</h1>
                <h3 className='date'>{page.date}</h3>
                <div className='description'>{nl2br(page.description)}</div>
                <div className='content'>{page.content}</div>
                <div className='links'>
                    <Button href={page.github}><img alt="GitHub" src={GitHub} className="mr-2"/>View on GitHub</Button>
                    <Button href={page.link}>{page.linkText}</Button>
                </div>
            </div>
        </div>;
    }

    renderPageNotFound() {
        return <>Page not found</>;
    }

    render() {
        const page = Projects.find(project => project.id === this.props.match.params.id);
        return page ? this.renderPage(page) : this.renderPageNotFound();
    }
}

ProjectPage.propTypes = {
    match: PropTypes.object,
};