# Actionable Assistant - Full Implementation Documentation

## Overview

The **Actionable Assistant** is a comprehensive system that enables users to execute file system operations through natural language commands. It provides a secure, auditable, and user-friendly way to automate file operations with full safety mechanisms, undo capabilities, and tamper-evident audit logging.

### Key Features

- ✅ **Natural Language Parsing** - Intent detection from user commands
- ✅ **File Operations** - Create, read, copy, move, delete files and directories
- ✅ **Path Flexibility** - Operations can be performed anywhere on the system (not limited to home directory)
- ✅ **Path Prompting** - Automatic prompts when file/directory paths are missing from commands
- ✅ **Risk Scoring** - Automatic risk assessment for all actions
- ✅ **Action Preview** - Preview actions before execution with granular approval
- ✅ **Snapshot & Undo** - Automatic snapshots for undo capability
- ✅ **Audit Logging** - Tamper-evident audit trail with hash chain
- ✅ **Security** - Path traversal prevention, capability tokens, validation
- ✅ **LLM Integration** - Uses selected AI provider for complex multi-step planning

---

## Architecture

### Tech Stack

**Backend (Rust)**
- Tauri 2.0 framework
- SQLite for persistent storage (`tauri-plugin-sql`)
- SHA-256 hashing for audit log integrity
- JSON Web Tokens for capability tokens

**Frontend (TypeScript/React)**
- React with TypeScript
- TailwindCSS for styling
- Radix UI components
- Tauri IPC for backend communication

**Database**
- SQLite with automatic migrations
- Tables: `action_snapshots`, `audit_logs`, `capability_tokens`

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (React/TypeScript)               │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ ActionMode   │  │ ActionPreview│  │ActionHistory │      │
│  │ Component   │  │  Component   │  │  Component   │      │
│  └──────┬───────┘  └──────┬─────────┘  └──────┬───────┘      │
│         │                 │                   │              │
│         └────────────────┴───────────────────┘              │
│                           │                                   │
│                    ┌──────▼────────┐                         │
│                    │useActionAssistant│                        │
│                    │     Hook       │                         │
│                    └──────┬─────────┘                         │
│                           │                                   │
└───────────────────────────┼───────────────────────────────────┘
                            │ IPC (Tauri Commands)
┌───────────────────────────▼───────────────────────────────────┐
│                  Backend (Rust)                                │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │            IPC Commands (commands.rs)                 │   │
│  │  • parse_intent                                       │   │
│  │  • plan_with_llm                                      │   │
│  │  • verify_action_plan                                │   │
│  │  • preview_action_plan                               │   │
│  │  • execute_action_plan                               │   │
│  │  • undo_action                                       │   │
│  │  • get_audit_history                                │   │
│  │  • mint_capability_token                             │   │
│  └───────────────┬──────────────────────────────────────┘   │
│                  │                                            │
│    ┌─────────────┼─────────────┬──────────────┐             │
│    │             │             │              │              │
│ ┌──▼──┐    ┌─────▼─────┐  ┌───▼────┐   ┌─────▼─────┐        │
│ │Planner│   │ Verifier  │  │Executor│   │  Policy   │        │
│ │       │   │           │  │        │   │           │        │
│ │•Determ│   │•Path Valid│  │•FS     │   │•Capability│        │
│ │ inistic│   │•Risk Score│  │ Adapter│   │  Tokens   │        │
│ │•LLM   │   │•Precond    │  │•Snapshot│  │           │        │
│ └───┬──┘   └───────────┘  │•Worker  │   └───────────┘        │
│     │                      └─────────┘                        │
│ ┌───▼──────────────────────────────────────────────┐         │
│ │           Audit Log (audit.rs)                    │         │
│ │  • Append-only log                                │         │
│ │  • Hash chain (SHA-256)                          │         │
│ │  • Tamper-evident                                 │         │
│ └──────────────────────────────────────────────────┘         │
└───────────────────────────────────────────────────────────────┘
                            │
                            │ SQL
┌───────────────────────────▼───────────────────────────────────┐
│                    SQLite Database                              │
│  • action_snapshots                                             │
│  • audit_logs                                                   │
│  • capability_tokens                                            │
└────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### Backend Modules

#### 1. Types (`assistant/types.rs`)

Defines all core data structures:
- `ActionSchema` - Action plan structure
- `Action` - Individual action with type, args, preconditions
- `ActionType` - Enum of supported operations
- `ActionPlan` - Complete plan before verification
- `VerifiedPlan` - Plan after verification
- `PreviewResult` - Preview data for UI
- `ActionResult` - Execution result
- `AuditEntry` - Audit log entry

#### 2. Planner (`assistant/planner/`)

**Deterministic Parser (`deterministic.rs`)**
- Rule-based intent extraction
- Pattern matching for common commands:
  - `create file X with content Y`
  - `read file X`
  - `copy file X to Y`
  - `move file X to Y`
  - `delete file X`
  - `create directory X`
- Detects missing paths and uses `__PROMPT_PATH__` placeholder
- Returns structured `ActionPlan`

**LLM Planner (`llm.rs`)**
- Placeholder for AI-driven planning
- Frontend handles LLM calls via `action-planner.function.ts`
- Uses user's selected AI provider

**Verifier (`verifier.rs`)**
- Validates action plans against schema
- Path validation (prevents traversal attacks)
- Precondition checking
- Risk score calculation (0.0 - 1.0)
- Skip validation for placeholder paths

#### 3. Executor (`assistant/executor/`)

**File System Adapter (`fs_adapter.rs`)**
- Safe file operations:
  - `create_file` - Creates file with content
  - `read_file` - Reads file content
  - `copy_file` - Copies file
  - `move_file` - Moves file
  - `delete_file` - Moves to trash (platform-specific)
  - `create_directory` - Creates directory
- No shell interpolation (direct Rust std::fs)

**Snapshot Manager (`snapshot.rs`)**
- Creates snapshots before file operations
- Stores in temporary directory
- Enables rollback/undo functionality
- Associates snapshots with action IDs

**Worker (`worker.rs`)**
- Orchestrates action execution
- Validates capability tokens
- Creates snapshots
- Executes actions transactionally
- Handles rollback on failure

#### 4. Policy (`assistant/policy.rs`)

**Capability Tokens**
- Minting with scopes and TTL
- Validation and verification
- Revocation support
- JWT-based implementation

#### 5. Audit Log (`assistant/audit.rs`)

**Tamper-Evident Logging**
- Append-only log
- SHA-256 hash chaining
- Each entry references previous entry's hash
- Signature generation for integrity verification
- Frontend handles database persistence

#### 6. Validator (`assistant/validator.rs`)

**Path Security**
- Validates paths to prevent traversal attacks
- Allows absolute paths anywhere on system
- Normalizes relative paths
- Prevents excessive `..` components
- Removed user home directory restriction

### Frontend Components

#### 1. `useActionAssistant` Hook

React hook providing:
- `parseIntent(userInput)` - Parse natural language
- `planAction(userInput, useLLM)` - Generate action plan
- `previewAction(plan)` - Get preview with risk score
- `executeAction(plan, confirmToken)` - Execute action
- `undoAction(actionId)` - Undo previous action
- `getAuditHistory(limit)` - Fetch audit log
- `mintToken(planId, scopes, ttl)` - Mint capability token

#### 2. ActionPreview Component

**Features:**
- Displays action plan summary
- Shows risk score with color coding
- Lists affected items (files/directories)
- Granular action selection (checkboxes)
- Missing path indicator and prompting
- Explicit confirmation for high-risk actions
- Approve/Reject/Edit buttons

**Path Prompting:**
- Automatically detects missing paths
- Opens `PathPromptDialog` when needed
- Prompts user sequentially for each missing path
- Updates action plan with provided paths

#### 3. ActionMode Component

Dedicated UI tab for action interface:
- Text input for commands
- Toggle for LLM planning
- Action preview display
- Execution status
- Error handling

#### 4. ActionHistory Component

**Features:**
- Displays audit log entries
- Shows action details (summary, files, timestamp)
- Undo button for each entry
- Scrollable history
- Loading and error states

#### 5. PathPromptDialog Component

**Features:**
- Input field for path (absolute paths supported)
- Browse button (disabled - requires plugin)
- Directory vs file mode
- Source vs destination mode
- Validation and error display
- Help text with path format examples

### Database Schema

**Migration: `db/migrations/assistant.sql`**

```sql
-- Action snapshots for undo functionality
CREATE TABLE IF NOT EXISTS action_snapshots (
    id TEXT PRIMARY KEY,
    action_id TEXT NOT NULL,
    snapshot_path TEXT NOT NULL,
    original_path TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

-- Tamper-evident audit log
CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    entry_json TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    prev_hash TEXT NOT NULL,
    signature TEXT NOT NULL,
    action_id TEXT,
    FOREIGN KEY (action_id) REFERENCES action_snapshots(action_id) ON DELETE SET NULL
);

-- Capability tokens
CREATE TABLE IF NOT EXISTS capability_tokens (
    id TEXT PRIMARY KEY,
    token TEXT NOT NULL,
    scopes TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);
```

---

## Action Schema v2

### Structure

```typescript
interface ActionPlan {
  id: string;
  origin: {
    user_input: string;
    source: "ui" | "voice" | "automation" | "plugin";
    request_id: string;
  };
  actions: Action[];
  summary: string;
  risk_score: number;  // 0.0 - 1.0
  dry_run: boolean;
}

interface Action {
  id: string;
  type: ActionType;
  args: Record<string, any>;
  preconditions?: {
    writable?: boolean;
    readable?: boolean;
    exists?: boolean;
    directory?: boolean;
  };
  metadata?: {
    confidence?: number;
  };
}
```

### Supported Action Types

- `fs_create_file` - Create a new file
- `fs_read_file` - Read file content
- `fs_copy_file` - Copy file to destination
- `fs_move_file` - Move file to destination
- `fs_delete_file` - Delete file (moves to trash)
- `fs_create_directory` - Create directory

---

## Security Features

### 1. Path Validation

**Prevents:**
- Path traversal attacks (`../../../etc/passwd`)
- Excessive parent directory references
- Invalid path structures

**Allows:**
- Absolute paths anywhere on system
- Relative paths (resolved from current working directory)
- Normal path operations

### 2. Capability Tokens

- Ephemeral, scoped permissions
- JWT-based with expiration
- Scope-based access control
- Revocation support

### 3. Risk Scoring

**Calculated based on:**
- Action type (delete = higher risk)
- Path location
- Preconditions
- Operation complexity

**Risk Levels:**
- Low (0.0 - 0.3): Safe operations (read, create)
- Medium (0.3 - 0.7): Moderate operations (copy, move)
- High (0.7 - 1.0): Dangerous operations (delete)

### 4. Audit Logging

**Features:**
- Append-only (immutable)
- Hash chain (each entry linked to previous)
- SHA-256 signatures
- Tamper detection
- Full action history

### 5. Transactional Execution

- Pre-action snapshots
- Rollback on failure
- Atomic operations
- Undo capability

---

## Usage Guide

### Basic Commands

**Create File:**
```
create file /path/to/file.txt with Hello World
```

**Read File:**
```
read file /path/to/file.txt
```

**Copy File:**
```
copy file /path/source.txt to /path/dest.txt
```

**Move File:**
```
move file /path/old.txt to /path/new.txt
```

**Delete File:**
```
delete file /path/to/delete.txt
```

**Create Directory:**
```
create directory /path/to/newfolder
```

### Missing Path Prompting

**When path is missing:**
```
create file
```
→ System detects missing path
→ Opens path prompt dialog
→ User provides path (e.g., `C:\Users\Name\file.txt`)
→ Action continues

### LLM Planning

For complex, multi-step operations:
1. Enable "Use AI for complex planning" in ActionMode
2. Type natural language request:
   ```
   analyze all markdown files in my documents and create a summary
   ```
3. LLM generates multi-step action plan
4. Preview shows all actions
5. Approve or reject

### Action Flow

```
User Input
    ↓
Intent Detection (Deterministic or LLM)
    ↓
Action Plan Generation
    ↓
Verification & Risk Scoring
    ↓
Preview Display (with missing path prompts if needed)
    ↓
User Approval
    ↓
Snapshot Creation
    ↓
Capability Token Validation
    ↓
Action Execution
    ↓
Audit Logging
    ↓
Result Returned
```

---

## Implementation Details

### Path Resolution

**Before (Home Directory Restricted):**
- All paths resolved relative to user home
- Absolute paths validated to be within home
- Security: Prevents access outside home

**After (System-Wide Access):**
- Absolute paths used as-is
- Relative paths resolved from current working directory
- Security: Prevents traversal attacks only
- Path prompting for missing paths

### Missing Path Detection

**Parser Logic:**
1. Detects commands without paths (e.g., `create file`)
2. Creates action with `__PROMPT_PATH__` placeholder
3. Verifier skips validation for placeholders
4. Preview shows "Path Information Needed"
5. User provides path via `PathPromptDialog`
6. Action plan updated with real path
7. Verification re-runs with actual path

### Audit Log Hash Chain

**Structure:**
```
Entry 1: signature = SHA256(entry1 + "")
Entry 2: signature = SHA256(entry2 + entry1.signature)
Entry 3: signature = SHA256(entry3 + entry2.signature)
...
```

**Verification:**
- Each entry references previous entry's signature
- Tampering breaks the chain
- Can verify integrity by checking hash chain

### Snapshot System

**Process:**
1. Before file operation: Create snapshot
2. Store snapshot path in database
3. Execute operation
4. On undo: Restore from snapshot
5. Snapshots expire after configurable TTL

**Storage:**
- Temporary directory (`tempfile` crate)
- Associated with action ID
- Can restore entire action or individual files

---

## Database Operations

### Frontend Database Actions

**File:** `lib/database/action-history.action.ts`

**Functions:**
- `saveAuditLogCache(entry)` - Save audit entry with hash chain
- `getAuditLogsFromDB(limit)` - Fetch audit history
- `saveSnapshot(snapshot)` - Save snapshot metadata
- `getSnapshotsByActionId(actionId)` - Get snapshots for undo
- `deleteSnapshot(snapshotId)` - Clean up expired snapshots

**Hash Chain Calculation:**
- Frontend calculates SHA-256 hashes using Web Crypto API
- Maintains chain by referencing previous entry's signature
- Ensures tamper-evident log

---

## Testing

### Manual Testing

**Test 1: Basic File Creation**
```
Input: create file /tmp/test.txt with Hello World
Expected: File created at /tmp/test.txt
```

**Test 2: Missing Path Prompting**
```
Input: create file
Expected: Dialog opens, user provides path, file created
```

**Test 3: Path Security**
```
Input: create file ../../../../etc/passwd
Expected: Rejected (path traversal detected)
```

**Test 4: Complex LLM Planning**
```
Input: organize my documents folder by creating year subdirectories
Expected: Multi-step plan generated, preview shown
```

**Test 5: Undo Functionality**
```
1. Create file
2. Check ActionHistory
3. Click Undo
Expected: File restored to previous state
```

### Security Testing

- ✅ Path traversal prevention
- ✅ Capability token validation
- ✅ Audit log integrity verification
- ✅ Snapshot cleanup

---

## Integration Points

### With Existing Chat System

**File:** `hooks/useCompletion.ts`

**Integration:**
- Detects action keywords in chat input
- Automatically parses intent
- Logs preview to console (UI integration pending)
- Falls through to normal chat if parsing fails

**Keywords Detected:**
- `create file`
- `read file`
- `copy file`
- `move file`
- `delete file`
- `create directory`
- etc.

### UI Integration

**Current State:**
- Components ready (`ActionMode`, `ActionPreview`, `ActionHistory`)
- Can be added to Settings panel or main UI
- Export available in `components/assistant/index.ts`

**To Add:**
```tsx
// In Settings or main UI
import { ActionMode, ActionHistory } from "@/components/assistant";

<ActionMode />
<ActionHistory />
```

---

## Future Enhancements

### Planned Features

1. **File Browser Integration**
   - Add `@tauri-apps/plugin-dialog` package
   - Enable browse button in `PathPromptDialog`
   - Native file picker integration

2. **Full Undo Implementation**
   - Complete snapshot lookup by action ID
   - Restore entire actions from audit log
   - Partial undo (individual files)

3. **OS-Specific Sandboxing**
   - Full sandbox implementation in `sandbox.rs`
   - Platform-specific isolation
   - Resource limits

4. **Enhanced LLM Planning**
   - Better prompt engineering
   - Multi-step validation
   - Plan optimization

5. **Action Templates**
   - Save common action plans
   - Quick action shortcuts
   - Custom action definitions

6. **Real-time Preview**
   - Live preview in chat
   - Auto-suggest actions
   - Inline action execution

7. **Voice Integration**
   - Voice command parsing
   - Audio feedback
   - Voice confirmation

---

## Known Limitations

1. **Browse Button**: Requires `@tauri-apps/plugin-dialog` (currently disabled)
2. **Undo Lookup**: Snapshot lookup by action ID needs database integration
3. **LLM Planning**: Prompt engineering may need refinement
4. **Sandboxing**: Basic structure only (full OS-specific sandboxing pending)
5. **Preview UI in Chat**: Detection works, but preview modal not integrated yet

---

## API Reference

### IPC Commands

**Backend → Frontend (via Tauri IPC):**

```rust
// Parse intent from user input
parse_intent(user_input: String) -> Result<ActionPlan, String>

// Plan with LLM (validates LLM-generated plan)
plan_with_llm(plan: ActionPlan) -> Result<VerifiedPlan, String>

// Verify action plan
verify_action_plan(plan: ActionPlan) -> Result<VerifiedPlan, String>

// Preview action plan
preview_action_plan(plan: ActionPlan) -> Result<PreviewResult, String>

// Execute action plan
execute_action_plan(plan: ActionPlan, confirm_token: String) -> Result<ActionResult, String>

// Undo action
undo_action(audit_entry_id: String) -> Result<ActionResult, String>

// Get audit history
get_audit_history(limit: i32) -> Result<Vec<AuditEntry>, String>

// Mint capability token
mint_capability_token(plan_id: String, scopes: Vec<String>, ttl_seconds: u64) -> Result<String, String>
```

### Frontend Hook API

```typescript
const {
  parseIntent,
  planAction,
  previewAction,
  executeAction,
  undoAction,
  getAuditHistory,
  mintToken,
  state,
  error,
} = useActionAssistant();
```

---

## Error Handling

### Common Errors

**"Could not parse intent"**
- Cause: Command doesn't match patterns
- Solution: Use more explicit command or enable LLM planning

**"Path outside allowed directory"** (removed)
- Previous: Paths restricted to home directory
- Current: Can use any path on system

**"Path traversal detected"**
- Cause: Malicious path with excessive `..`
- Solution: Use valid paths without traversal

**"Precondition failed"**
- Cause: File should exist but doesn't (or vice versa)
- Solution: Check file state before action

**"Failed to save audit log"**
- Cause: Database error
- Solution: Check database connection, restart app

---

## Performance Considerations

- **Path Validation**: O(n) where n = path length
- **Hash Chain**: O(1) per entry (constant time)
- **Snapshot Creation**: O(size) for file size
- **Audit Log Query**: O(limit) for history retrieval
- **Verification**: O(actions) per plan

---

## Security Best Practices

1. **Always verify paths** before execution
2. **Use capability tokens** for scoped permissions
3. **Maintain audit log** for accountability
4. **Calculate risk scores** for all actions
5. **Require confirmation** for high-risk operations
6. **Use snapshots** for undo capability
7. **Validate preconditions** before executing

---

## Conclusion

The Actionable Assistant provides a complete, secure, and user-friendly system for file operations through natural language. It combines:

- **Safety**: Path validation, risk scoring, snapshots
- **Usability**: Natural language, path prompting, preview
- **Auditability**: Tamper-evident logs, full history
- **Flexibility**: System-wide access, LLM planning, extensibility

The system is production-ready for basic operations and can be extended with additional features as needed.

---

## File Structure

```
scribe/
├── src-tauri/src/assistant/
│   ├── types.rs              # Core data structures
│   ├── mod.rs                # Module declarations
│   ├── commands.rs           # IPC command handlers
│   ├── validator.rs          # Path validation
│   ├── policy.rs             # Capability tokens
│   ├── audit.rs              # Audit logging
│   ├── sandbox.rs            # Sandboxing (placeholder)
│   ├── planner/
│   │   ├── mod.rs
│   │   ├── deterministic.rs  # Rule-based parser
│   │   ├── llm.rs            # LLM planner (placeholder)
│   │   └── verifier.rs       # Plan verification
│   └── executor/
│       ├── mod.rs
│       ├── fs_adapter.rs     # File operations
│       ├── snapshot.rs       # Snapshot management
│       └── worker.rs         # Execution orchestrator
├── src/
│   ├── components/assistant/
│   │   ├── ActionMode.tsx
│   │   ├── ActionPreview.tsx
│   │   ├── ActionHistory.tsx
│   │   ├── PathPromptDialog.tsx
│   │   └── index.ts
│   ├── hooks/
│   │   └── useActionAssistant.ts
│   ├── lib/
│   │   ├── functions/
│   │   │   └── action-planner.function.ts
│   │   └── database/
│   │       └── action-history.action.ts
│   └── types/
│       └── assistant.ts
└── src-tauri/src/db/migrations/
    └── assistant.sql
```

---

**Last Updated:** Current implementation status
**Version:** 1.0.0
**Status:** ✅ Core features implemented and functional

