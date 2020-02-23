import React from 'react';
import PropTypes from 'prop-types';
import { NavLink } from 'react-router-dom';

import './button.scss';

export default class Button extends React.Component {
    render() {
        if (this.props.to)
            return <NavLink className='button' to={this.props.to}>{this.props.children}</NavLink>;
        if (this.props.href)
            return <a className='button' href={this.props.href}>{this.props.children}</a>;
        if (this.props.onClick)
            return <div className='button' onClick={this.props.onClick}>{this.props.children}</div>;
        return null;
    }
}

Button.propTypes = {
    children: PropTypes.any,
    to: PropTypes.string,
    onClick: PropTypes.func,
    href: PropTypes.string,
};
