import React from 'react';
import PropTypes from 'prop-types';
import { Button as ButtonBootstrap } from 'react-bootstrap';
import { NavLink } from 'react-router-dom';


export default class Button extends React.Component {
    render() {
        if (this.props.to)
            return <ButtonBootstrap className={`mx-1 ${this.props.className}`} variant={this.props.variant} to={this.props.to} as={NavLink}>{this.props.children}</ButtonBootstrap>;
        if (this.props.href)
            return <ButtonBootstrap className={`mx-1 ${this.props.className}`} variant={this.props.variant} href={this.props.href}>{this.props.children}</ButtonBootstrap>;
        if (this.props.onClick)
            return <ButtonBootstrap className={`mx-1 ${this.props.className}`} variant={this.props.variant} onClick={this.props.onClick}>{this.props.children}</ButtonBootstrap>;
        return null;
    }
}

Button.propTypes = {
    children: PropTypes.any,
    to: PropTypes.string,
    onClick: PropTypes.func,
    variant: PropTypes.string,
    href: PropTypes.string,
};
