import * as sapper from '@sapper/app';
import './routes/projects/_projects';

sapper.start({
	target: document.querySelector('#sapper')
});