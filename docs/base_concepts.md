# Base Concepts

## Mime Component
Instead of **Spring Bean**: an object that is created, managed, and destroyed by the Spring IoC (Inversion of Control) container. It is simply a regular Java object whose lifecycle and dependencies are controlled by Spring.
1. Beans are instantiated by the Spring container, not by calling new.
2. Dependencies are injected (constructor, setter, or field injection).
3. Beans are defined using:
    * `@Component`, `@Service`, `@Repository`, `@Controller`
    * or `@Bean` methods inside a `@Configuration` class
    * or XML configuration (older style)
4. The container manages:
    * lifecycle (*initialization*, *destruction*)
    * scopes (*singleton*, *prototype*, *request*, *session*, etc.):
    * dependency wiring
5. `BeanFactory`
6. `BeanWrapper`

### Scopes:
* **Singleton**:
    * **Spring**: One instance per container (application-wide).
    * **Rust equivalent**: Store the instance in an `Arc` (or `Rc`) inside your container struct so that all consumers share the same object.
        ```rust
        let repo = Arc::new(InMemoryUserRepository);
        let service = Arc::new(UserServiceImpl { repo: repo.clone() }); // singleton service
        ```
        The `Arc` ensures multiple owners, similar to Spring's singleton bean.
* **Scoped (per request / per session)**:
    * **Spring**: New instance per HTTP request or per session.
    * **Rust equivalent**: You create a new instance each time the “scope” starts, passing it to the consumer. In a web server, this often maps to **per-request state**, usually using frameworks like Actix or Axum.
        ```rust
        fn handle_request() {
            let repo = InMemoryUserRepository; // fresh instance for request
            let service = UserServiceImpl { repo };
            // use service
        }
        ```
        Lifetime is naturally bounded by the function scope. No global container needed.
* **Transient (prototype)**:
    * **Spring**: New instance every time the bean is injected.
    * **Rust equivalent**: Simply create a new object wherever needed.
        ```rust
        let service1 = UserServiceImpl { repo: InMemoryUserRepository };
        let service2 = UserServiceImpl { repo: InMemoryUserRepository };
        // service1 and service2 are independent
        ```
        Ownership and Rust's borrow checker make this safe without GC.