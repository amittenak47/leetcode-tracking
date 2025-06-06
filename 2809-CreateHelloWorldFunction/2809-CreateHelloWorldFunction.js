// Last updated: 6/6/2025, 3:31:10 PM
/**
 * @return {Function}
 */
var createHelloWorld = function() {
    
    return function(...args) {
         return "Hello World"
    }
};

/**
 * const f = createHelloWorld();
 * f(); // "Hello World"
 */