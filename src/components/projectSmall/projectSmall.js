import React from 'react';
import PropTypes from 'prop-types';
import { Card, Button } from 'react-bootstrap';
import { NavLink } from 'react-router-dom';


const overflow = (text, limit) => `${text.slice(0, limit)}${text.length > limit ? '...' : ''}`;

export default class ProjectSmall extends React.Component {
    renderPreview() {
        if (!this.props.image) {
            return;
        }
        if (this.props.image.type) {
            return this.props.image;
        } else {
            return <Card.Img variant="top" src={this.props.image} alt={this.props.image} className='image'/>;
        }
    }

    render(){
        return <Card>
            <Card.Header>
                {this.props.tags.join(', ')}
            </Card.Header>
            {this.renderPreview()}
            <Card.Body>
                <Card.Title>{this.props.title}</Card.Title>
                <Card.Subtitle>{this.props.date}</Card.Subtitle>
                <Card.Text>{overflow(this.props.description, 150)}</Card.Text>
                <Button to={this.props.id} variant="link" as={NavLink}>View</Button>
            </Card.Body>
        </Card>;
    }
}

ProjectSmall.propTypes = {
    id: PropTypes.string.isRequired,
    title: PropTypes.string.isRequired,
    date: PropTypes.string,
    description: PropTypes.string,
    tags: PropTypes.array,
    image: PropTypes.any,
    history: PropTypes.object.isRequired,
    delay: PropTypes.number,
    visible: PropTypes.bool,
    searchTag: PropTypes.func,
};
