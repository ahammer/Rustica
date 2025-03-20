# Definition of Done (DoD)

This document outlines the criteria that must be met for any work item to be considered "Done" in the Rustica game engine project. These criteria ensure quality, maintainability, and consistency across the project.

## General Acceptance Criteria

A feature, task, or bug fix is considered "Done" when:

1. **Functionality is complete**
   - All acceptance criteria specified in the ticket/issue are met
   - Implementation satisfies the stated requirements
   - Edge cases have been considered and handled

2. **Code Quality Standards**
   - Code follows the [Implementation Rules](../docs/implementation_rules.md)
   - No compiler warnings or errors
   - Code is properly formatted according to project style guidelines
   - No TODOs without associated tickets (except those explicitly allowed)

3. **Testing**
   - Unit tests written and passing (minimum 80% coverage for new code)
   - Integration tests written where appropriate and passing
   - System tests for end-to-end functionality are passing
   - All tests can run in headless mode for CI
   - Performance critical code has benchmark tests

4. **Documentation**
   - Public API is documented with Rust doc comments
   - Complex algorithms or non-obvious behavior is explained inline
   - Changes to architecture are reflected in architecture documentation
   - Module-specific guide is updated if needed

5. **Review**
   - Code has been reviewed by at least one other team member
   - All review comments have been addressed or discussed
   - Review confirmed adherence to implementation and API conventions

6. **Integration**
   - Implementation does not break existing functionality
   - Changes integrate smoothly with the rest of the codebase
   - No regressions detected in existing test suite

## Process Guidelines

### Stay Focused
- Work on tasks according to their priority
- Complete tasks incrementally rather than taking on multiple large changes
- If a task is too large, break it down into smaller subtasks

### Don't Get Distracted
- Avoid unplanned work or scope creep
- If you identify additional work needed, create a new ticket for it
- Stay within the boundaries of the current sprint

### Communicate Blockers
- If you're blocked on a task, communicate it early
- Don't sit on a problem; seek help when needed
- Document any external dependencies or blockages

### Test Your Work
- Write tests before or alongside implementation (TDD preferred)
- Verify that tests actually test what they claim to test
- Include both positive tests (expected behavior) and negative tests (error handling)

### Track Your Progress
- Update task status regularly in the sprint tracker
- Document any important decisions or approaches taken
- Record time spent on tasks for future estimation reference

## Definition of Ready

Before a task is considered "Ready" to be worked on, it must:

1. Have clear acceptance criteria
2. Be properly sized (not too large for a single sprint)
3. Have dependencies identified and available
4. Be prioritized relative to other backlog items

## Done Checklist Template

Use this checklist when completing a task:

- [ ] All acceptance criteria met
- [ ] Code follows implementation rules
- [ ] No compiler warnings or errors
- [ ] Unit tests written and passing
- [ ] Integration/system tests where applicable
- [ ] Documentation updated
- [ ] Code reviewed
- [ ] No regressions in existing functionality
- [ ] Task tracking updated
