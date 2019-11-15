import React from 'react';
import PropTypes from 'prop-types';

export default class LichessBoard extends React.Component {
    render() {
        return <iframe
            src={`https://lichess.org/${this.props.url}`}
            title={this.props.url}
            width={600}
            height={397}
            frameBorder={0}
        ></iframe>;
    }
}

LichessBoard.propTypes = {
    url: PropTypes.string.isRequired,
};