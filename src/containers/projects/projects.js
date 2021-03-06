import React from 'react';
import PropTypes from 'prop-types';
import { Helmet } from 'react-helmet';

import projects from 'content/projects';
import ProjectSmall from 'components/projectSmall/projectSmall';
import { Container, CardColumns } from 'react-bootstrap';

// import Search from 'components/search/search';

export default class Projects extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            search: '',
        };
    }

    handleSearch(search) {
        this.setState({ search });
    }

    render() {
        const filterBySearch = proj => proj.search.includes(this.state.search);
        return <>
            <Helmet>
                <title>Projects | Ruairidh Williamson</title>
                <meta name='description' content=''/>
            </Helmet>
            {/* <Search onChange={this.handleSearch.bind(this)} value={this.state.search} placeholder='Search...'/> */}
            <Container className="mt-3">
                <CardColumns>
                    {projects.map((project, index) => <ProjectSmall
                        key={project.id}
                        history={this.props.history}
                        delay={index}
                        visible={filterBySearch(project)}
                        searchTag={this.handleSearch.bind(this)}
                        {...project}
                    />)}
                </CardColumns>
            </Container>
        </>;
    }
}

Projects.propTypes = {
    location: PropTypes.object,
    history: PropTypes.object,
};