import React from 'react';
import PropTypes from 'prop-types';

export default class PreloadImages extends React.Component {
    render() {
        return this.props.images.filter((item, i) => this.props.images.indexOf(item) === i).filter(href => !!href).map(href => <link key={href} rel='prefetch' href={href} as='image'/>);
    }
}

PreloadImages.propTypes = {
    images: PropTypes.array.isRequired,
};
