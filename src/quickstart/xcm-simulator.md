### xcm-simulator
Setting up a live network with multiple connected parachains for testing XCM is not straight forward. The `XCM-simulator` was created as a solution to this problem. The XCM-simulator is a network simulator specifically designed for testing and playing around with XCM. It uses mock relay chain and parachain runtime. 

For testing xcm configurations for live runtime environments we use the `XCM-emulator`. The XCM-emulator can use production relay chain and parachain runtimes. Users can plug in Kusama, Statemine, or their custom runtime etc. With up-to-date chain specs, it's able to verify if specific XCM messages work in live networks. The specific use cases will be further explained in the chapter on [testing](testing/README.md).

In the next section we will take a first look at an XCM. The XCM-simulator is used for the example code.

[Next: First Look at an XCM](first-look.md)





