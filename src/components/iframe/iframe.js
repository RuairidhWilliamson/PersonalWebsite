import React from 'react';
import PropTypes from 'prop-types';

export default class Iframe extends React.Component {
    render() {
        return <iframe
            style={{ width: '100%', minHeight: '1000px', height: '-webkit-fill-available', ...this.props.style }}
            title={this.props.title || this.props.src}
            src={this.props.src}
            frameBorder="0"
            marginHeight="0"
            marginWidth="0"
            scrolling={this.props.scrolling ? 'yes' : 'no'}
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
    scrolling: PropTypes.bool,
    className: PropTypes.string,
};

Iframe.defaultProps = {
    scrolling: false,
};