import React from 'react';
import PropTypes from 'prop-types';

import './section.scss';

export default class Section extends React.Component {
    render(){
        return <div className='section'>
            <h2 className='label'>{this.props.label}</h2>
            <div className='content'>
                {this.props.children}
            </div>
        </div>;
    }
}

Section.propTypes = {
    label: PropTypes.string,
    children: PropTypes.any,
};
