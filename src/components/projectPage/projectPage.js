import React from 'react';
import PropTypes from 'prop-types';

import Projects from 'content/projects';
import './projectPage.scss';

export default class ProjectPage extends React.Component {

    renderPage(page) {
        return <div className='project-page'>
            <div className='project'>
                <h1 className='title'>{page.title}</h1>
                <h4 className='date'>{page.date}</h4>
                <div className='description'>{page.description}</div>
                <div className='content'>{page.content}</div>
            </div>
        </div>;
    }

    renderPageNotFound() {
        return <>Page not found</>;
    }

    render() {
        const page = Projects.find(project => project.id === this.props.match.params.id);
        console.log(page);
        return page ? this.renderPage(page) : this.renderPageNotFound();
    }
}

ProjectPage.propTypes = {
    match: PropTypes.object,
};