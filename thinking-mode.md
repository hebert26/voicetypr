═══════════════════════════════════════════════════════════════════════════════
                    ULTRATHINK TIPS & TRICKS FOR CLAUDE CODE
═══════════════════════════════════════════════════════════════════════════════

1. PROGRESSIVE ESCALATION STRATEGY
───────────────────────────────────
   Start low, escalate only if needed:
   
   First attempt:  "think about how to solve this bug"         → 4k tokens
   If stuck:       "think hard about this architecture"        → 10k tokens
   Complex stuff:  "ultrathink this migration strategy"        → 32k tokens

2. BEST USE CASES FOR ULTRATHINK
────────────────────────────────
   ✓ Complex architectural decisions
   ✓ Breaking out of repetitive loops (when Claude keeps failing)
   ✓ Multi-step refactoring across many files
   ✓ Security reviews and audits
   ✓ Debugging elusive/complex bugs
   ✓ Planning migrations or major rewrites
   ✓ Unfamiliar codebase analysis

3. WHEN TO AVOID ULTRATHINK
───────────────────────────
   ✗ Simple code changes or quick fixes
   ✗ Routine tasks you do often
   ✗ Every single prompt (burns tokens fast)
   ✗ Tasks where speed matters more than depth

4. COMBINE WITH PLAN MODE
─────────────────────────
   Use Shift+Tab to enter Plan Mode, then add ultrathink:
   
   "ultrathink and create a detailed plan for implementing 
    OAuth2 authentication - don't write code yet"

5. PAIR WITH SUBAGENTS FOR COMPLEX TASKS
────────────────────────────────────────
   "ultrathink this problem and use subagents to investigate 
    the authentication module and the database schema separately"

6. VIEW CLAUDE'S THINKING
─────────────────────────
   Press Ctrl+O to expand the condensed thinking blocks
   and see Claude's actual reasoning process

7. CLEAR CONTEXT BEFORE BIG THINKS
──────────────────────────────────
   Run /clear before a complex ultrathink task
   → Gives Claude maximum context space for deep reasoning
   → Avoids wasted tokens on irrelevant history

8. EXPLICIT VERIFICATION REQUESTS
─────────────────────────────────
   "ultrathink this solution, verify your assumptions, 
    and check for edge cases before implementing"

9. COST AWARENESS
─────────────────
   Use /cost to monitor token usage
   Ultrathink at 32k tokens is ~8x more expensive than basic think
   
   Rough estimates per ultrathink task: $0.30-0.50

10. TRIGGER PHRASES THAT WORK
─────────────────────────────
    All of these trigger maximum 32k budget:
    • ultrathink
    • think harder
    • think intensely  
    • think longer
    • think really hard
    • think super hard
    • think very hard

═══════════════════════════════════════════════════════════════════════════════
                              EXAMPLE WORKFLOWS
═══════════════════════════════════════════════════════════════════════════════

DEBUGGING A TRICKY BUG:
  1. First: "think about why this test is failing"
  2. If no luck: "think hard, read the related files, what am I missing?"
  3. Nuclear option: "ultrathink - analyze the entire flow and find the bug"

ARCHITECTURE DECISION:
  "ultrathink: I need to choose between microservices and a modular 
   monolith for this project. Consider our team size, deployment 
   constraints, and scaling needs. Create a decision matrix."

BREAKING A LOOP:
  When Claude keeps making the same mistake:
  "STOP. ultrathink about what's going wrong. You've tried this 
   approach 3 times. What fundamental assumption is incorrect?"

═══════════════════════════════════════════════════════════════════════════════
