import projects from '../src/routes/projects/_projects';

test('there are more than 3 projects', () => {
    expect(projects.length).toBeGreaterThan(3);
});

describe.each(projects.map(project => [project.slug, project]))('Project %s', (_, project) => {
    test(`has valid slug`, () => {
        expect(project.slug).toBeTruthy();
        expect(project.slug).toMatch(/^[a-z0-9\-]+$/);
    });

    test(`has a valid title`, () => {
        expect(project.title).toBeTruthy();
    });

    test(`has a valid date`, () => {
        expect(project.date).toBeTruthy();
    });

    test(`has valid html`, () => {
        expect(project.html).toBeTruthy();
    });

    test(`has tags`, () => {
        expect(project.tags).toBeTruthy();
    });
});

test('Each project should have a unique slug', () => {
    projects.forEach(project => {
        const count = projects.filter(b => b.slug == project.slug).reduce((prev) => prev + 1, -1);
        expect(count).toBe(0);
    });
});