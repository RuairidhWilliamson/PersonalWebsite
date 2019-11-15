import React from 'react';
import PropTypes from 'prop-types';

export default class Iframe extends React.Component {
    render() {
        return <iframe
            style={{ ...this.props.style }}
            title={this.props.title || this.props.src}
            src={this.props.src}
            frameBorder="0"
            marginHeight="0"
            marginWidth="0"
            scrolling="no"
            className={this.props.className}
        >
            {this.props.children}
        </iframe>;
    }
}

Iframe.propTypes = {
    src: PropTypes.string.isRequired,
    title: PropTypes.string,
    style: PropTypes.object,
    children: PropTypes.any,
    width: PropTypes.any,
    height: PropTypes.any,
    className: PropTypes.string,
};