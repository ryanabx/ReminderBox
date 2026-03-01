![ReminderBox Logo](./reminderbox.svg)

<p align="center">
<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/ryanabx/ReminderBox/gh-pages-deploy.yml">
</p>

## A (WIP) PWA reminders app in your browser!

ReminderBox is a reminders app I developed mainly for personal use, but since it's a web app, you can use it too!

## Reminders are stored locally

Since it's a single-page application that runs entirely on the client, it's explicitly a non-goal to have any sort of database or server in the stack. All reminders are stored on the [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage). This also means you shouldn't clear the website's local storage, or else you'll lose your reminders!

In the future, I may implement a backup and restore option for reminders, just in case you frequently clear your website data.

## For contributors

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more information on how to compile!