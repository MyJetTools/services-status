
var gulp = require('gulp');
var minifyjs = require('gulp-js-minify');
var concat = require('gulp-concat');

gulp.task('default', function () {
    return gulp
        .src(['./JavaScript/HtmlStaticElement.js',
            './JavaScript/HtmlMain.js',
            './JavaScript/HtmlStatusBar.js',
            './JavaScript/main.js'])
        .pipe(minifyjs())
        .pipe(concat('app.js'))
        .pipe(gulp.dest('./wwwroot/js/'))
});