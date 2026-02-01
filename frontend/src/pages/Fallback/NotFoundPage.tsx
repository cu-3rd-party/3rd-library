import { useNavigate } from "react-router-dom";

const NotFoundPage = () => {
  const navigate = useNavigate();

  return <div>Страница не найдено</div>;
};

export default NotFoundPage;
