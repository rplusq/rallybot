# WhatsApp Bot Specification

## Overview
A WhatsApp bot for joining padel sessions. Users can browse available sessions by type and register for them.

## Core Functionality

### 1. Start Conversation

#### For Registered Users
When user says "hey", "hi", "hello" or 🎾:
```
👋 Hey [Name]! What type of session would you like to join?

C - Coaching Classes
S - Social Games
L - League Games
X - Mixed levels Social Games
0️⃣ Show the events I'm registered to

👉 Reply with the right letter to see available sessions or 0 to show your upcoming sessions!
```

#### For Unregistered Users
When an unregistered user (not in database by phone number) sends any message:
```
Hi there! It looks like you're not registered with our community yet. Only registered members can interact with me and sign up for Rally events.

🎾 Want to join the fun? Apply to become a member using the link below.

Once your application is approved, you'll be ready to hit the court and Rally with us! ✨
```

### 2. List Sessions
When user replies with C, S, L, or X, show available sessions:
```
Thanks for choosing! Here are the [Session Type] available this week:

1️⃣ ⏰ Mon 30 10:00 📍 Sports Center A 
🎯 Upper-Intermediate  
👤 Player 1
👤 Player 2
👤 Player 3
👤 Player 4  
    Substitutes:
🎾 Player 5
🎾 Player 6
🎾 Player 7 

2️⃣ ⏰ Tue 1 11:30 📍 Sports Center B 
[truncated with "Read more"]
```

Empty state:
```
Sorry, there are no [Session Type] available this week...
```

### 3. Join Session
When user replies with a number (1, 2, etc.):
```
✅ Congratulations, [Name]! You're signed up!

[Session Type]
Level: [Skill Level]
⏰ [Day Date Time] 📍 [Venue]
[Player list]
```

If full:
```
⚠️ This event is currently full.
📋 You've been added to the substitutes list! If a spot opens up, I'll notify you right away.
```

### 4. Show My Sessions
When user replies with 0:
```
Here are your upcoming events:

Social Games
1️⃣ ⏰ Sat 28 18:00 📍 Sports Center C
🎯 Upper-Intermediate
[Player list with user highlighted]

League Games
[Additional sessions]
```

### 5. Error Handling
For any unrecognized command:
```
Sorry, I don't understand that command ! Press 🎾 to see the menu
```

## Technical Notes

- Extract user name from WhatsApp profile
- Track conversation state to know which menu was shown
- Sessions have 4 player slots
- Full sessions show substitute list
- Add to calendar functionality mentioned

## Message Flow

### For Registered Users
1. User: "hey", "hi", "hello", or 🎾
2. Bot: Shows main menu
3. User: Selects session type (C/S/L/X) or my sessions (0)
4. Bot: Lists available sessions or user's sessions
5. User: Selects session number
6. Bot: Confirms registration or adds to waitlist

If user sends unrecognized text:
- Bot: "Sorry, I don't understand that command ! Press 🎾 to see the menu"
- User: 🎾
- Bot: Shows main menu again

### For Unregistered Users
1. User: Any message
2. Bot: Shows registration required message with application link
3. All subsequent messages receive the same registration required response