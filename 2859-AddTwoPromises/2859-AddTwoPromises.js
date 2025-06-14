// Last updated: 6/6/2025, 3:31:07 PM
/**
 * @param {Promise} promise1
 * @param {Promise} promise2
 * @return {Promise}
 */
var addTwoPromises = async function(promise1, promise2) 
{
    try 
    {
        return promise1.then((val) => promise2.then((val2) => val + val2))
    } 
    catch (error) 
    {
        console.error(error);
        throw error; 
    }    
};

/**
 * addTwoPromises(Promise.resolve(2), Promise.resolve(2))
 *   .then(console.log); // 4
 */