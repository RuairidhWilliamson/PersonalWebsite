import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';

import Header from 'containers/header/header';
import ProjectPage from 'components/projectPage/projectPage';
import PreloadImage from 'components/preloadImages/PreloadImages';
import Pages from 'content/pages';
import Projects from 'content/projects';

import 'bootstrap/dist/css/bootstrap.min.css';

export default class Root extends React.Component {
    render() {
        return (
            <Router>
                <PreloadImage images={Projects.map(el => el.image)}/>
                <div className='page-container'>
                    <Route component={Header}/>
                    <div className='page'>
                        <Switch>
                            {Pages.map(page => <Route key={page.label} exact={page.exact} path={page.path} label={page.label} component={page.component}/>)}
                            <Route path='/:id' component={ProjectPage}/>
                        </Switch>
                    </div>
                </div>
            </Router>
        );
    }
}