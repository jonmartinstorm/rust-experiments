# Testing channels with async functions

This simple and silly program runs three different tasks with different time to finish as async threads. 

* The simulated Unix task uses 1 second to finish
* The simulated TCP task uses 2 seconds to finish
* The simulated simulation task uses 3 seconds to finish

The program runs a loop once every 100ms for 2.5 seconds. In each iteration if the loop it starts one of each task.
After the loop the program waits for 3 seconds so the last started simulation task has time to finish.

I have also a multi-producer single consumer channel to send some information of each task to the main function.