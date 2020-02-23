import React from 'react';
import PropTypes from 'prop-types';

import './materialIcon.scss';

export default function Icon(props) {
    return props.icon ? <i onClick={props.onClick} className={`material-icons ${props.className}`} style={props.style}>{props.icon}</i> : null;
}

Icon.propTypes = {
    icon: PropTypes.string,
    onClick: PropTypes.func,
    className: PropTypes.string,
    style: PropTypes.object,
};
