# Changelog

All notable changes to this project will be documented in this file.

## [0.4.1] - 2024-02-23

### Features

- Get_multi_latest_trades functionality
- Get_multi_trades functionality
- BarSession trait for bars module
- OTO ordering
- All methods return a Result type, add a changelog, clean up

### Miscellaneous Tasks

- Readme
- Specify pkg version for ureq
- Specify pkg version for serde_json
- Change pkg version for serde_json
- Change catagories
- Update readme
- Update readme
- Update README
- Pkg version update
- Comments and documentation
- Comments and documentation
- Comments and documentation
- Comments and documentation
- Comments and documentation
- Update versions
- Update RS docs
- Update examples in README and update dependacies to latest minors

### Refactor

- Proper error handling for the order module

### Add

- Trend to Bar struct

# 🚀 Changelog

## All notable changes to this project will be documented in this file.

## [0.3.2] - 2023-07-02

### 📝 Chore

- 📝 Update examples in README

### ⬆️

- ⬆️ Update dependencies to latest minors

---

## [0.3.1] - 2023-07-02

### 📝 Chore

- 📝 RS Docs cleanup

---

## [0.3.0] - 2023-07-02

### ✨ Added

- 🌟 A CHANGELOG

### 🔄 Changed

- 🔄 All methods from all modules now return a Result type. Users can now implement their own error handling
- 🔄 get_trade_activity is now get_trade_activity_by_type to allow filtering of a specific type, even though all that alpaca has for now is "FILL"

---

## [0.2.1] - 2023-06-24

### 🔄 Changed

- 🔄 Order module methods now return a Result type. Users can now implement their own error handling

---

## [0.2.0] - 2023-06-24 📦

### ✨ Added

- 🌟 One Triggers Other orders for stop loss and take profit

---

## [0.1.6] - 2023-06-22 📦

### ✨ Added

- 🌟 pub type Bars now has a trait that enables quick methods for getting the highs, lows, closes, and opens.
- 🌟 Improve initial documentation

---

## [0.1.3] - 2023-06-21 📦

### ✨ Added

- 🌟 Get latest trades for multiple stocks

---

## [0.1.0] - 2023-06-19 🎉

### ✨ Added

- 🎈 Initial release

- 🚀 Features included:
  - Get trades
  - Get bars
  - Place orders
  - Get positions
  - Get activity
  - Get account

---

## 🌐 Template

### ✨ Added

- 🚀 Any new features are documented here

### 🐞 Fixed

- 🩹 Any bugs or issues that were addressed are documented here

### 🔄 Changed

- 🔄 Any changes or improvements to existing features are documented here

### 🚫 Removed

- ❌ Any deprecated features or removed elements are documented here

### 🔒 Security

- 🛡️ Any updates or improvements to security are documented here
