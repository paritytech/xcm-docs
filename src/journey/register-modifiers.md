# Register Modifiers
In the previous chapters we already saw instructions that modified the XCVM registers. This chapter contains more instructions that change the XCVM registers. We will discuss the following instructions: 
- `SetErrorHandler`
- `SetAppendixHandler`
- `ClearError`
- `ClearTransactStatus`
- `SetTopic`
- `ClearTopic`

## SetErrorHandler
```rust
SetErrorHandler(Xcm<Call>)
```
The `SetErrorHandler` instructions is used to set the Error Handler Register. As discussed in the [XCVM chapter](TODO), the Error Handler is executed when an error is thrown during the regular instruction execution. 

## SetAppendixHandler
```rust
SetAppendixHandler(Xcm<Call>)
```
The `SetAppendixHandler` instruction is used to set the Appendix Handler Register. As discussed in the [XCVM chapter](TODO), the Appendix Handler instructions are executed after the regular and error handler instruction are executed. These instructions are executed regardless of whether an error occurred. 

## ClearError
```rust
ClearError
```
The `ClearError` instruction clears the Error Register. More specifically, it sets the Error Register to None. 

## ClearTransactStatus
```rust
ClearTransactStatus
```
The `ClearTransactStatus` instruction sets the Transact Status Register to its default, cleared, value.

## SetTopic
```rust
SetTopic([u8; 32])
```
The `SetTopic` instruction sets the Topic Register.

## ClearTopic
```rust
ClearTopic
```
The `ClearTopic` instruction clears the Topic Register.

