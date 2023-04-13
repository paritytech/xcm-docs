### xcm-emulator
Setting up a live network with multiple connected parachains for testing XCM is not straight forward. The XCM-emulator was created as a solution to this problem. The XCM-emulator is a network emulator specifically designed for testing and playing around with XCM. It emulates the sending, delivery and execution of XCM instructions, with the assumption that the message can always be delivered to and executed in the destination. 

The xcm-emulator can use production relay chain and parachain runtimes. Users can plug in Kusama, Statemine, or their custom runtime etc. With up-to-date chain specs, it's able to verify if specific XCM messages work in live networks.

The emulator makes it possible to quickly set up a custom emulated network. [This example](TODO) shows how to setup a network of one relaychain and three parachains. 



